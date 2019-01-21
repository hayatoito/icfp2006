// For icfp2006 VM implementation.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <arpa/inet.h>

/* #define CDSIZE 15923865 */
/* #define PROGNAME "umix.umz" */

//#define CDSIZE 2357032
//#define PROGNAME "codex.umz"

/* #define CDSIZE 56364 */
/* #define PROGNAME "sandmark.umz" */

//#define CDSIZE 6260
//#define PROGNAME "um6hello.um"

//#define BIGDEBUG
//#define LOGOUT

#ifdef BIGDEBUG
#define DEBN(n) {fprintf(stderr, "%08x ", n);}
#define DEBS(s) {fprintf(stderr, "%s\n",   s);}
#else
#define DEBN(n) {}
#define DEBS(s) {}
#endif

#define MAXARR (128 * 1024 * 1024)

typedef unsigned int u32;

static void load(const char* progname, u32 **ptr, u32*siz) {
  FILE *f;
  int i;
  u32 *p;
  f = fopen(progname, "r");
  fseek(f, 0, SEEK_END);
  int cdsize = ftell(f);
  fseek(f, 0, SEEK_SET);
  *siz = cdsize / 4;
  *ptr = p = (u32 *) malloc(sizeof(u32) * cdsize);
  fread(ptr[0], sizeof(u32), cdsize, f);
  fclose(f);

  for(i = 0 ; i < cdsize; ++i){
    p[i] = htonl(p[i]);
  }
}


#define STKMAX (1024*1024)

static u32 stk1[STKMAX];
static u32 stkp = 0;

static u32 pop(u32 *pn) {
  if(stkp > 0){
    return stk1[--stkp];
  }
  *pn = (*pn) + 1;
  return *pn;
}

static void push(u32 old) {
  if(stkp < (STKMAX - 1)) stk1[stkp++] = old;
}

int main(int argc, char *argv[]) {
  static u32 *ptr[MAXARR];
  static u32  siz[MAXARR];
  static u32  reg[8];
  u32 pc, pn, nc;
  u32 i;

#ifdef LOGOUT
  FILE *logfile;
  logfile = fopen("debug_log","wb");
  setbuf(logfile, 0);
#endif

  if (argc == 0) {
    printf("code is not given");
    exit(1);
  }
  /* if (argc == 2) { */
  /*   // TODO(hayato): open file as stdin. */
  /*   FILE *stdinfile; */
  /*   stdinfile = fopen(argv[2],"rb"); */
  /*   setbuf(stdinfile, 0); */
  /* } */


  load(argv[1], &ptr[0], &siz[0]);

  setbuf(stdin,   0);
  setbuf(stdout,  0);

  for(i = 0 ; i < 8 ; i++) reg[i] = 0;
  pc = 0;
  pn = 1;
  nc = 0;

  // int index = 0;

  for(;;){
    /* if (index++ == 10000000) { */
    /*   fprintf(stderr, "early exit\n"); */
    /*   return 1; */
    /* } */

    u32 op,ir,a,b,c;

    ir = ptr[0][pc];
    op = ((ir >> (32 - 4)));
    a  = ((ir >> 6) & 7);
    b  = ((ir >> 3) & 7);
    c  = ((ir >> 0) & 7);

    //if(!(nc & 0x00ffffff)) fprintf(stderr, "nc %08x pc %08x ir %08x\n", nc, pc, ir);

#ifdef BIGDEBUG
    for(i = 0 ; i < 8 ; i++) DEBN(reg[i]);
    DEBS("");
    DEBN(a);
    DEBN(b);
    DEBN(c);
    DEBN(pc);
#endif

    pc++;
    nc++;

    switch(op){
    case 0:
      DEBS("CMOV\n");
      if(reg[c]) reg[a] = reg[b];
      break;
    case 1:
      DEBS("LD\n");
      reg[a] = ptr[reg[b]][reg[c]];
      break;
    case 2:
      DEBS("ST\n");
      ptr[reg[a]][reg[b]] = reg[c];
      break;
    case 3:
      DEBS("ADD\n");
      reg[a] = reg[b] + reg[c];
      break;
    case 4:
      DEBS("MUL\n");
      reg[a] = reg[b] * reg[c];
      break;
    case 5:
      DEBS("DIV\n");
      reg[a] = reg[b] / reg[c];
      break;
    case 6:
      DEBS("NAND\n");
      reg[a] = ~(reg[b] & reg[c]);
      break;
    case 7:
      DEBS("ABORT\n");
      fprintf(stderr, "ABORT reached\n");
      return 0;
      break;
    case 8:
      DEBS("ALLOC\n");
      if(pn == MAXARR){
	fprintf(stderr, "MAXARR reached\n");
	return 1;
      }
      i = pop(&pn);
      //fprintf(stderr, "alloc %d size %d\n", i, reg[c]);
      siz[i] = reg[c];
      ptr[i] = (u32 *) calloc(sizeof(u32), siz[i]);
      reg[b] = i;
      pn++;
      break;
    case 9:
      DEBS("FREE\n");
      //fprintf(stderr, "free %d\n", reg[c]);
      free(ptr[reg[c]]);
      push(reg[c]);
      break;
    case 10:
      DEBS("PUTCHAR\n");
      if(reg[c] > 255) abort(); else{
	fputc(reg[c], stdout);
#ifdef LOGOUT
	fputc(reg[c], logfile);
#endif
      }
      break;
    case 11:
      DEBS("GETCHAR\n");
      {
        // TODO(hayato): read from file at first, then stdin,
	reg[c] = fgetc(stdin);
      }
      break;
    case 12:
      DEBS("LOAD\n");
      if(reg[b]){
	const int m = siz[reg[b]];
	free(ptr[0]);
	ptr[0] = (u32 *) malloc(sizeof(u32) * m);
	memcpy(ptr[0], ptr[reg[b]], sizeof(u32) * m);
	siz[0] = m;
      }
      pc = reg[c];
      break;
    case 13:
      DEBS("IMM\n");
      a = (ir >> 25) & 7;
      reg[a] = ir & 0x01ffffff;
      break;
    }
  }

  return 0;
}
