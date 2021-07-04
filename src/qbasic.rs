use crate::um::Um;
use crate::um::UmStatus;
use lazy_static::lazy_static;

use crate::prelude::*;
use std::io::Read;

// See task/02_qbasic/output.txt

fn number_to_roman(n: u32) -> &'static str {
    const ROMAN_CHARS: [char; 5] = ['V', 'X', 'L', 'C', 'D'];
    const MAX: u32 = 200;

    assert!(n > 0);
    assert!(n < MAX);

    lazy_static! {
        static ref NUMBER_TO_ROMAN: Vec<String> = {
            // digits:
            //     V  X   L  C   D
            //     1, 2  10 20 100
            // e.g.
            // 0: [0, 0, 0, 0, 0]
            // V: [1, 0, 0, 0, 0 ]
            // X: [0, 1, 0, 0, 0 ]
            // XV: [1, 1, 0, 0, 0 ]
            // XX: [0, 2, 0, 0, 0 ]
            (0..MAX).scan([0; ROMAN_CHARS.len()], |digits, _| {

                if digits[0] < 1 {
                    digits[0] += 1;
                } else {
                    assert_eq!(digits[0], 1);
                    digits[0] = 0;

                    if digits[1] < 3 {
                        digits[1] += 1;

                        // [0, 0, 0, 4, 0] -> [0, 0, 0, -1, 1]
                        if digits[3] == 4 {
                            digits[3] = -1;
                            digits[4] += 1;
                        }

                    } else {
                        assert_eq!(digits[1], 3);
                        digits[1] = -1;

                        if digits[2] < 1 {
                            digits[2] += 1;
                        } else {
                            assert_eq!(digits[2], 1);
                            digits[2] = 0;
                            if digits[3] < 3 {
                                digits[3] += 1;
                            } else {
                                assert_eq!(digits[3], 3);

                                // Prevent [?, -1, ?, -1, ?], in favor of [?, -1, ?, 4, ?]
                                if digits[1] == -1 {
                                    digits[3] += 1;
                                } else {
                                    digits[3] = -1;
                                    digits[4] += 1;
                                    assert!(digits[4] < 2);
                                }
                            }
                        }
                    }
                }

                // println!("{} -> {:?}", n, digits);

                let mut roman = String::new();
                for (i, d) in digits.iter().enumerate().rev() {
                    if *d == -1 {
                        assert!(roman.len() > 0);
                        // 98 -> [0, -1, 0, 0, 1] => XD (wrong) =>  (correct) CDXC
                        // [?, -1, 0, 0, 1] => [?, -1, 0, (1,-1), 1]
                        if i + 2 < digits.len() && digits[i + 1] == 0 && digits[i + 2] == 0 {
                            roman.insert(roman.len() - 1, ROMAN_CHARS[i + 2]);
                            roman.push(ROMAN_CHARS[i + 2]);
                        }

                        roman.insert(roman.len() - 1, ROMAN_CHARS[i]);
                        continue;
                    }
                    for _ in 0..*d {
                        roman.push(ROMAN_CHARS[i]);
                    }
                }
                Some(roman)
            }).collect()
        };
    }
    &NUMBER_TO_ROMAN[(n - 1) as usize]
}

pub fn qbasic() -> Vec<String> {
    let program1 = r#"
V        REM  +------------------------------------------------+
X        REM  | HACK.BAS      (c) 19100   fr33 v4r14bl3z       |
XV       REM  |                                                |
XX       REM  | Brute-forces passwords on UM vIX.0 systems.    |
XXV      REM  | Compile with Qvickbasic VII.0 or later:        |
XXX      REM  |    /bin/qbasic hack.bas                        |
XXXV     REM  | Then run:                                      |
XL       REM  |   ./hack.exe username                          |
XLV      REM  |                                                |
L        REM  | This program is for educational purposes only! |
LV       REM  +------------------------------------------------+
LX       REM
LXV      IF ARGS() > I THEN GOTO LXXXV
LXX      PRINT "usage: ./hack.exe username"
LXXV     PRINT CHR(X)
LXXX     END
LXXXV    REM
XC       REM  get username from command line
XCV      DIM username AS STRING
C        username = ARG(II)
CV       REM  common words used in passwords
CX       DIM pwdcount AS INTEGER
CXV      pwdcount = LIII
CXX      DIM words(pwdcount) AS STRING
CXXV     words(I) = "airplane"
CXXX     words(II) = "alphabet"
CXXXV    words(III) = "aviator"
CXL      words(IV) = "bidirectional"
CXLV     words(V) = "changeme"
CL       words(VI) = "creosote"
CLV      words(VII) = "cyclone"
CLX      words(VIII) = "december"
CLXV     words(IX) = "dolphin"
CLXX     words(X) = "elephant"
CLXXV    words(XI) = "ersatz"
CLXXX    words(XII) = "falderal"
CLXXXV   words(XIII) = "functional"
CXC      words(XIV) = "future"
CXCV     words(XV) = "guitar"
CC       words(XVI) = "gymnast"
CCV      words(XVII) = "hello"
CCX      words(XVIII) = "imbroglio"
CCXV     words(XIX) = "january"
CCXX     words(XX) = "joshua"
CCXXV    words(XXI) = "kernel"
CCXXX    words(XXII) = "kingfish"
CCXXXV   words(XXIII) = "(\b.bb)(\v.vv)"
CCXL     words(XXIV) = "millennium"
CCXLV    words(XXV) = "monday"
CCL      words(XXVI) = "nemesis"
CCLV     words(XXVII) = "oatmeal"
CCLX     words(XXVIII) = "october"
CCLXV    words(XXIX) = "paladin"
CCLXX    words(XXX) = "pass"
CCLXXV   words(XXXI) = "password"
CCLXXX   words(XXXII) = "penguin"
CCLXXXV  words(XXXIII) = "polynomial"
CCXC     words(XXXIV) = "popcorn"
CCXCV    words(XXXV) = "qwerty"
CCC      words(XXXVI) = "sailor"
CCCV     words(XXXVII) = "swordfish"
CCCX     words(XXXVIII) = "symmetry"
CCCXV    words(XXXIX) = "system"
CCCXX    words(XL) = "tattoo"
CCCXXV   words(XLI) = "thursday"
CCCXXX   words(XLII) = "tinman"
CCCXXXV  words(XLIII) = "topography"
CCCXL    words(XLIV) = "unicorn"
CCCXLV   words(XLV) = "vader"
CCCL     words(XLVI) = "vampire"
CCCLV    words(XLVII) = "viper"
CCCLX    words(XLVIII) = "warez"
CCCLXV   words(XLIX) = "xanadu"
CCCLXX   words(L) = "xyzzy"
CCCLXXV  words(LI) = "zephyr"
CCCLXXX  words(LII) = "zeppelin"
CCCLXXXV words(LIII) = "zxcvbnm"
CCCXC    REM try each password
CCCXCV   PRINT "attempting hack with " + pwdcount + " passwords " + CHR(X)
CD       DIM i AS INTEGER
CDV      i = I
CDX      IF CHECKPASS(username, words(i)) THEN GOTO CDXXX
CDXV     i = i + I
CDXX     IF i > pwdcount THEN GOTO CDXLV
CDXXV    GOTO CDX
CDXXX    PRINT "found match!! for user " + username + CHR(X)
CDXXXV   PRINT "password: " + words(i) + CHR(X)
CDXL     END
CDXLV    PRINT "no simple matches for user " + username + CHR(X)
CDL      REM
CDLV     REM  the above code will probably crack passwords for many
CDLX     REM  users so I always try it first. when it fails, I try the
CDLXV    REM  more expensive method below.
CDLXX    REM
CDLXXV   REM  passwords often take the form
CDLXXX   REM    dictwordDD
CDLXXXV  REM  where DD is a two-digit decimal number. try these next:
"#;

    // PRINT "checkpass: " + words(i) + CHR(XLVII + j) + CHR(XLVII + k) + CHR(X)

    let program2_src = r#"
DIM j AS INTEGER
DIM k AS INTEGER
i = I
REM for-i
j = I
REM for-j
k = I
REM for-k
IF CHECKPASS(username, words(i) + CHR(XLVII + j) + CHR(XLVII + k)) THEN GOTO end1
GOTO inc-k
REM end1
PRINT "found match!! for user " + username + CHR(X)
PRINT "password: " + words(i) + CHR(XLVII + j) + CHR(XLVII + k) + CHR(X)
END
REM inc-k
k = k + I
IF k > X THEN GOTO inc-j
GOTO for-k
REM inc-j
j = j + I
IF j > X THEN GOTO inc-i
GOTO for-j
REM inc-i
i = i + I
IF i > pwdcount THEN GOTO nomatch
GOTO for-i
REM nomatch
PRINT "no matches for user " + username + CHR(X)
"#;

    let program2_offset = program1.trim().split('\n').count();
    let program2_src = program2_src.trim().split('\n').collect::<Vec<_>>();

    let roman = (1..(program2_offset + program2_src.len() + 2))
        .map(|n| number_to_roman(n as u32))
        .collect::<Vec<_>>();

    // 1st scan: collect (label to line) mapping
    let mut label_line = std::collections::HashMap::new();
    for (i, line) in program2_src.iter().enumerate() {
        if line.starts_with("REM") {
            label_line.insert(&line[4..], program2_offset + 1 + i);
        }
    }

    // 2nd scan
    let goto_re = regex::Regex::new("(.*GOTO )(.*)$").unwrap();
    let program2 = program2_src
        .iter()
        .enumerate()
        .map(|(i, line)| {
            format!(
                "{:8} {}",
                &roman[program2_offset + 1 + i],
                if let Some(caps) = goto_re.captures(line) {
                    let label = &caps[2];
                    let line_num: usize = label_line[label];
                    goto_re
                        .replace(line, |caps: &regex::Captures| {
                            format!("{}{}", &caps[1], &roman[line_num])
                        })
                        .to_string()
                } else {
                    line.to_string()
                }
            )
        })
        .collect::<Vec<_>>();

    let mut program1 = program1
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    program1.extend(program2);
    program1
}

/**
[2020-07-14 Tue] Result summary:

found match!! for user ftd
password: falderal90

found match!! for user ohmega
password: bidirectional

found match!! for user howie
password: xyzzy
**/
pub fn solve(code: String) -> Result<()> {
    // Get a list of usernames

    // % ls /home
    // ls /home
    // ftd/
    // knr/
    // guest/
    // gardener/
    // ohmega/
    // yang/
    // howie/
    // hmonk/
    // bbarker/

    // Solve the qbasic
    let mut f = std::fs::File::open(code)?;
    let mut code = Vec::new();
    f.read_to_end(&mut code)?;

    let input = format!(
        "guest
cd code
/bin/umodem myhack.bas STOP
{}
STOP
/bin/qbasic myhack.bas
./myhack.exe ftd
./myhack.exe knr
./myhack.exe gardener
./myhack.exe ohmega
./myhack.exe yang
./myhack.exe howie
./myhack.exe hmonk
./myhack.exe bbarker
",
        qbasic().join("\n")
    );

    let mut um = Um::new(code);
    um.set_print_stdin(true);
    let result = um.run(&mut input.as_bytes(), &mut std::io::stdout());
    assert_eq!(result, UmStatus::NoInput);

    let result = um.continue_with(&mut std::io::stdin(), &mut std::io::stdout());
    assert_eq!(result, UmStatus::NoInput);

    // um.set_print_stdin(false);
    Ok(())

    /*
    Output:

    You have new mail. Type 'mail' to view.
    % ./myhack.exe ftd
    attempting hack with LIII passwords
    no simple matches for user ftd
    found match!! for user ftd
    password: falderal90

    You have new mail. Type 'mail' to view.
    % ./myhack.exe knr
    attempting hack with LIII passwords
    no simple matches for user knr
    no matches for user knr

    You have new mail. Type 'mail' to view.
    % ./myhack.exe gardener
    attempting hack with LIII passwords
    no simple matches for user gardener
    no matches for user gardener

    You have new mail. Type 'mail' to view.
    % ./myhack.exe ohmega
    attempting hack with LIII passwords
    found match!! for user ohmega
    password: bidirectional

    You have new mail. Type 'mail' to view.
    % ./myhack.exe yang
    attempting hack with LIII passwords
    no simple matches for user yang
    no matches for user yang

    You have new mail. Type 'mail' to view.
    % ./myhack.exe howie
    attempting hack with LIII passwords
    found match!! for user howie
    password: xyzzy

    You have new mail. Type 'mail' to view.
    % ./myhack.exe hmonk
    attempting hack with LIII passwords
    no simple matches for user hmonk
    no matches for user hmonk

    You have new mail. Type 'mail' to view.
    % ./myhack.exe bbarker
    attempting hack with LIII passwords
    no simple matches for user bbarker
    no matches for user bbarker

    You have new mail. Type 'mail' to view.
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_to_roman_test() {
        assert_eq!(number_to_roman(1), "V");
        assert_eq!(number_to_roman(2), "X");
        assert_eq!(number_to_roman(3), "XV");

        let roman_answers = vec![
            "V", "X", "XV", "XX", "XXV", "XXX", "XXXV", "XL", "XLV", "L", "LV", "LX", "LXV", "LXX",
            "LXXV", "LXXX", "LXXXV", "XC", "XCV", "C", "CV", "CX", "CXV", "CXX", "CXXV", "CXXX",
            "CXXXV", "CXL", "CXLV", "CL", "CLV", "CLX", "CLXV", "CLXX", "CLXXV", "CLXXX", "CLXXXV",
            "CXC", "CXCV", "CC", "CCV", "CCX", "CCXV", "CCXX", "CCXXV", "CCXXX", "CCXXXV", "CCXL",
            "CCXLV", "CCL", "CCLV", "CCLX", "CCLXV", "CCLXX", "CCLXXV", "CCLXXX", "CCLXXXV",
            "CCXC", "CCXCV", "CCC", "CCCV", "CCCX", "CCCXV", "CCCXX", "CCCXXV", "CCCXXX",
            "CCCXXXV", "CCCXL", "CCCXLV", "CCCL", "CCCLV", "CCCLX", "CCCLXV", "CCCLXX", "CCCLXXV",
            "CCCLXXX", "CCCLXXXV", "CCCXC", "CCCXCV", "CD", "CDV", "CDX", "CDXV", "CDXX", "CDXXV",
            "CDXXX", "CDXXXV", "CDXL", "CDXLV", "CDL", "CDLV", "CDLX", "CDLXV", "CDLXX", "CDLXXV",
            "CDLXXX", "CDLXXXV", "CDXC", "CDXCV", "D",
        ];

        for (i, roman) in roman_answers.iter().enumerate() {
            // println!("{} == {}", i + 1, roman);
            assert_eq!(number_to_roman((i + 1) as u32), *roman);
        }
    }

    #[test]
    fn qbasic_test() {
        for line in qbasic() {
            println!("{}", line);
        }
    }

    /*
    100 D
    101 DV
    102 DX
    103 DXV
    104 DXX
    105 DXXV
    106 DXXX
    107 DXXXV
    108 DXL
    109 DXLV
    110 DL
    111 DLV
    112 DLX
    113 DLXV
    114 DLXX
    115 DLXXV
    116 DLXXX
    117 DLXXXV
    118 DXC
    119 DXCV
    120 DC
    121 DCV
    122 DCX
    123 DCXV
    124 DCXX
    125 DCXXV
    126 DCXXX
    127 DCXXXV
    128 DCXL
    129 DCXLV
    130 DCL
    131 DCLV
    132 DCLX
    133 DCLXV
    134 DCLXX
    135 DCLXXV
    136 DCLXXX
    137 DCLXXXV
    138 DCXC
    139 DCXCV
    140 DCC
    141 DCCV
    142 DCCX
    143 DCCXV
    144 DCCXX
    145 DCCXXV
    146 DCCXXX
    147 DCCXXXV
    148 DCCXL
    149 DCCXLV
    150 DCCL
    151 DCCLV
    152 DCCLX
    153 DCCLXV
    154 DCCLXX
    155 DCCLXXV
    156 DCCLXXX
    157 DCCLXXXV
    158 DCCXC
    159 DCCXCV
    160 DCCC
    161 DCCCV
    162 DCCCX
    163 DCCCXV
    164 DCCCXX
    165 DCCCXXV
    166 DCCCXXX
    167 DCCCXXXV
    168 DCCCXL
    169 DCCCXLV
    170 DCCCL
    171 DCCCLV
    172 DCCCLX
    173 DCCCLXV
    174 DCCCLXX
    175 DCCCLXXV
    176 DCCCLXXX
    177 DCCCLXXXV
    178 DCCCXC
    179 DCCCXCV
    180 DCD
    181 DCDV
    182 DCDX
    183 DCDXV
    184 DCDXX
    185 DCDXXV
    186 DCDXXX
    187 DCDXXXV
    188 DCDXL
    189 DCDXLV
    190 DCDL
    191 DCDLV
    192 DCDLX
    193 DCDLXV
    194 DCDLXX
    195 DCDLXXV
    196 DCDLXXX
    197 DCDLXXXV
    198 DCDXC
    199 DCDXCV
    */
}
