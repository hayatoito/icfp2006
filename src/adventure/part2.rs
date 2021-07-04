use crate::prelude::Result;

use crate::adventure::*;
use crate::um::Um;
use crate::um::UmStatus;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::io::Read;

// TODO: move common code from part1.rs to adventure.rs

// use std::collections::*;

// downloader:
// - [ ] USB cable
// - [ ] display
// - [ ] jumper shunt
// - [ ] progress bar
// - [ ] power cord

// uploader:
// - [ ] ...

// [2019-09-09 Mon] Try to fix "USB Cable"

// 54th Street and Dorchester Avenue

// For part 2
// Go(Direction),  // Use this later
// TakeAt(Location, String),

// For part2
type Location = (usize, usize);

#[derive(Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "north",
                Direction::South => "south",
                Direction::East => "east",
                Direction::West => "west",
            }
        )
    }
}

/*
>: l
l
54th Street and Ridgewood Court

You are standing at the corner of 54th Street and Ridgewood
Court. From here, you can go east.
There is a /etc/passwd here.
Underneath the /etc/passwd, there is a self-addressed note.
Underneath the note, there is a (broken) downloader.
Underneath the downloader, there is a (broken) uploader.

>: l /etc/passwd
l /etc/passwd
The /etc/passwd is some kind of lost inode. It reads:
howie:xyzzy:Howard Curry:/home/howie
yang:U+262F:Y Yang:/home/yang
hmonk:COMEFROM:Harmonious Monk:/home/hmonk.
Also, it is in pristine condition.

>: l note
l note
The note is written in a familiar hand.
It reads: Dear Self, I had to erase our memory to protect the
truth. The Municipality has become more powerful than we had
feared. Its Censory Engine has impeded the spread of information
throughout our ranks. I've left two useful items for you here,
but I had to disassemble them and scatter the pieces. Each piece
may be assembled from the items at a single location. Repair the
items and recover the blueprint from the Museum of Science and
Industry; it will show you how to proceed. If you have trouble
reading the blueprint, know that the Censory Engine blocks only
your perception, not your actions. Have courage, my self, the
abstraction is weak! P.S. SWITCH your GOGGLES!. Interestingly,
this one is self-addressed.
Also, it is in pristine condition.

>: l downloader
l downloader
The downloader is (according to the label) fully compatible with
third generation municipal robots.
Also, it is broken: it is a downloader missing a USB cable and a
display and a jumper shunt and a progress bar and a power cord.

>: l uploader
l uploader
The uploader is used to update firmware on municipal robots. A
label reads, Warning: use of this device will void your robot's
warranty.
Also, it is broken: it is an uploader missing a MOSFET and a
status LED and a RS232 adapter and a EPROM burner and a battery.



>: switch sexp
switch sexp
(success (command (switch "sexp")))
l
l
(success (command (look (room (name "54th Street and Ridgewood Court")(description "You are standing at the corner of 54th Street and Ridgewood Court. From here, you can go east. ")(items ((item (name "/etc/passwd")(description "some kind of lost inode. It reads:
howie:xyzzy:Howard Curry:/home/howie
yang:U+262F:Y Yang:/home/yang
hmonk:COMEFROM:Harmonious Monk:/home/hmonk")(adjectives )(condition (pristine ))(piled_on ((item (name "note")(description "written in a familiar hand.
It reads: Dear Self, I had to erase our memory to protect the truth. The Municipality has become more powerful than we had feared. Its Censory Engine has impeded the spread of information throughout our ranks. I've left two useful items for you here, but I had to disassemble them and scatter the pieces. Each piece may be assembled from the items at a single location. Repair the items and recover the blueprint from the Museum of Science and Industry; it will show you how to proceed. If you have trouble reading the blueprint, know that the Censory Engine blocks only your perception, not your actions. Have courage, my self, the abstraction is weak! P.S. SWITCH your GOGGLES!")(adjectives ((adjective "self-addressed") ))(condition (pristine ))(piled_on ((item (name "downloader")(description "(according to the label) fully compatible with third generation municipal robots")(adjectives )(condition (broken (condition (pristine ))(missing ((kind (name "USB cable")(condition (pristine ))) ((kind (name "display")(condition (pristine ))) ((kind (name "jumper shunt")(condition (pristine ))) ((kind (name "progress bar")(condition (pristine ))) ((kind (name "power cord")(condition (pristine ))) ))))))))(piled_on ((item (name "uploader")(description "used to update firmware on municipal robots. A label reads, Warning: use of this device will void your robot's warranty")(adjectives )(condition (broken (condition (pristine ))(missing ((kind (name "MOSFET")(condition (pristine ))) ((kind (name "status LED")(condition (pristine ))) ((kind (name "RS232 adapter")(condition (pristine ))) ((kind (name "EPROM burner")(condition (pristine ))) ((kind (name "battery")(condition (pristine ))) ))))))))(piled_on )) ))) ))) ))) ))))))



(success
 (command
  (look
   (room
    (name "54th Street and Ridgewood Court")
    (description "You are standing at the corner of 54th Street and Ridgewood Court. From here, you can go east. ")
    (items
     (
      (item
       (name "/etc/passwd")
       (description "some kind of lost inode. It reads:
howie:xyzzy:Howard Curry:/home/howie
yang:U+262F:Y Yang:/home/yang
hmonk:COMEFROM:Harmonious Monk:/home/hmonk")
       (adjectives )
       (condition
        (pristine ))
       (piled_on
        (
         (item
          (name "note")
          (description "written in a familiar hand.
It reads: Dear Self, I had to erase our memory to protect the truth. The Municipality has become more powerful than we had feared. Its Censory Engine has impeded the spread of information throughout our ranks. I've left two useful items for you here, but I had to disassemble them and scatter the pieces. Each piece may be assembled from the items at a single location. Repair the items and recover the blueprint from the Museum of Science and Industry; it will show you how to proceed. If you have trouble reading the blueprint, know that the Censory Engine blocks only your perception, not your actions. Have courage, my self, the abstraction is weak! P.S. SWITCH your GOGGLES!")
          (adjectives
           (
            (adjective "self-addressed") ))
          (condition
           (pristine ))
          (piled_on
           (
            (item
             (name "downloader")
             (description "
(according to the label) fully compatible with third generation municipal robots")
             (adjectives )
             (condition
              (broken
               (condition
                (pristine ))
               (missing
                (
                 (kind
                  (name "USB cable")
                  (condition
                   (pristine )))
                 (
                  (kind
                   (name "display")
                   (condition
                    (pristine )))
                  (
                   (kind
                    (name "jumper shunt")
                    (condition
                     (pristine )))
                   (
                    (kind
                     (name "progress bar")
                     (condition
                      (pristine )))
                    (
                     (kind
                      (name "power cord")
                      (condition
                       (pristine ))) ))))))))
             (piled_on
              (
               (item
                (name "uploader")
                (description "used to update firmware on municipal robots. A label reads, Warning: use of this device will void your robot's warranty")
                (adjectives )
                (condition
                 (broken
                  (condition
                   (pristine ))
                  (missing
                   (
                    (kind
                     (name "MOSFET")
                     (condition
                      (pristine )))
                    (
                     (kind
                      (name "status LED")
                      (condition
                       (pristine )))
                     (
                      (kind
                       (name "RS232 adapter")
                       (condition
                        (pristine )))
                      (
                       (kind
                        (name "EPROM burner")
                        (condition
                         (pristine )))
                       (
                        (kind
                         (name "battery")
                         (condition
                          (pristine ))) ))))))))
                (piled_on )) ))) ))) ))) ))))))


>: east
east
54th Street and Dorchester Avenue

You are standing at the corner of 54th Street and Dorchester
Avenue. From here, you can go north, east, south, or west.
There is an orange-red X-9247-GWE here.
Underneath the X-9247-GWE, there is a (broken) magenta
V-0010-XBD.
Underneath the V-0010-XBD, there is a pumpkin F-1403-QDS.
Underneath the F-1403-QDS, there is a (broken) heavy P-5065-WQO.

Underneath the P-5065-WQO, there is a taupe B-4832-LAL.
Underneath the B-4832-LAL, there is a (broken) gray40
L-6458-RNH.
Underneath the L-6458-RNH, there is a (broken) eggplant
T-9887-OFC.
Underneath the T-9887-OFC, there is a (broken) indigo
Z-1623-CEK.
Underneath the Z-1623-CEK, there is a yellow-green H-9887-MKY.
Underneath the H-9887-MKY, there is a (broken) shiny F-6678-DOX.

Underneath the F-6678-DOX, there is a pale-green R-1403-SXU.
Underneath the R-1403-SXU, there is a (broken) USB cable.
Underneath the USB cable, there is a sienna N-4832-NUN.
Underneath the N-4832-NUN, there is a slate-gray J-9247-IRG.
Underneath the J-9247-IRG, there is a dim-gray B-5065-YLQ.



>: l P-5065-WQO
l P-5065-WQO
The P-5065-WQO is an exemplary instance of part number
P-5065-WQO. Interestingly, this one is heavy.
Also, it is broken: it is ((a P-5065-WQO missing a T-6678-BTV)
missing a B-4832-LAL) missing a F-1403-QDS.



>: switch sexp
switch sexp
(success (command (switch "sexp")))
l
l
(success (command (look (room (name "54th Street and Dorchester Avenue")(description "You are standing at the corner of 54th Street and Dorchester Avenue. From here, you can go north, east, south, or west. ")(items ((item (name "X-9247-GWE")(description "an exemplary instance of part number X-9247-GWE")(adjectives ((adjective "orange-red") ))(condition (pristine ))(piled_on ((item (name "V-0010-XBD")(description "an exemplary instance of part number V-0010-XBD")(adjectives ((adjective "magenta") ))(condition (broken (condition (pristine ))(missing ((kind (name "X-9247-GWE")(condition (pristine ))) ))))(piled_on ((item (name "F-1403-QDS")(description "an exemplary instance of part number F-1403-QDS")(adjectives ((adjective "pumpkin") ))(condition (pristine ))(piled_on ((item (name "P-5065-WQO")(description "an exemplary instance of part number P-5065-WQO")(adjectives ((adjective "heavy") ))(condition (broken (condition (broken (condition (broken (condition (pristine ))(missing ((kind (name "T-6678-BTV")(condition (pristine ))) ))))(missing ((kind (name "B-4832-LAL")(condition (pristine ))) ))))(missing ((kind (name "F-1403-QDS")(condition (pristine ))) ))))(piled_on ((item (name "B-4832-LAL")(description "an exemplary instance of part number B-4832-LAL")(adjectives ((adjective "taupe") ))(condition (pristine ))(piled_on ((item (name "L-6458-RNH")(description "an exemplary instance of part number L-6458-RNH")(adjectives ((adjective "gray40") ))(condition (broken (condition (pristine ))(missing ((kind (name "P-5065-WQO")(condition (broken (condition (pristine ))(missing ((kind (name "T-6678-BTV")(condition (pristine ))) ))))) ))))(piled_on ((item (name "T-9887-OFC")(description "an exemplary instance of part number T-9887-OFC")(adjectives ((adjective "eggplant") ))(condition (broken (condition (broken (condition (pristine ))(missing ((kind (name "X-6458-TIJ")(condition (pristine ))) ))))(missing ((kind (name "H-9887-MKY")(condition (pristine ))) ))))(piled_on ((item (name "Z-1623-CEK")(description "an exemplary instance of part number Z-1623-CEK")(adjectives ((adjective "indigo") ))(condition (broken (condition (broken (condition (pristine ))(missing ((kind (name "D-4292-HHR")(condition (pristine ))) ))))(missing ((kind (name "L-6458-RNH")(condition (pristine ))) ))))(piled_on ((item (name "H-9887-MKY")(description "an exemplary instance of part number H-9887-MKY")(adjectives ((adjective "yellow-green") ))(condition (pristine ))(piled_on ((item (name "F-6678-DOX")(description "an exemplary instance of part number F-6678-DOX")(adjectives ((adjective "shiny") ))(condition (broken (condition (broken (condition (pristine ))(missing ((kind (name "J-9247-IRG")(condition (pristine ))) ))))(missing ((kind (name "V-0010-XBD")(condition (pristine ))) ))))(piled_on ((item (name "R-1403-SXU")(description "an exemplary instance of part number R-1403-SXU")(adjectives ((adjective "pale-green") ))(condition (pristine ))(piled_on ((item (name "USB cable")(description "compatible with all high-speed Universal Sand Bus 2.0 devices")(adjectives )(condition (broken (condition (broken (condition (broken (condition (pristine ))(missing ((kind (name "T-9887-OFC")(condition (broken (condition (pristine ))(missing ((kind (name "X-6458-TIJ")(condition (pristine ))) ))))) ))))(missing ((kind (name "F-6678-DOX")(condition (pristine ))) ))))(missing ((kind (name "N-4832-NUN")(condition (pristine ))) ))))(piled_on ((item (name "N-4832-NUN")(description "an exemplary instance of part number N-4832-NUN")(adjectives ((adjective "sienna") ))(condition (pristine ))(piled_on ((item (name "J-9247-IRG")(description "an exemplary instance of part number J-9247-IRG")(adjectives ((adjective "slate-gray") ))(condition (pristine ))(piled_on ((item (name "B-5065-YLQ")(description "an exemplary instance of part number B-5065-YLQ")(adjectives ((adjective "dim-gray") ))(condition (pristine ))(piled_on )) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))))))



(success
 (command
  (look
   (room
    (name "54th Street and Dorchester Avenue")
    (description "You are standing at the corner of 54th Street and Dorchester Avenue. From here, you can go north, east, south, or west. ")
    (items
     (
      (item
       (name "X-9247-GWE")
       (description "an exemplary instance of part number X-9247-GWE")
       (adjectives
        (
         (adjective "orange-red") ))
       (condition
        (pristine ))
       (piled_on
        (
         (item
          (name "V-0010-XBD")
          (description "an exemplary instance of part number V-0010-XBD")
          (adjectives
           (
            (adjective "magenta") ))
          (condition
           (broken
            (condition
             (pristine ))
            (missing
             (
              (kind
               (name "X-9247-GWE")
               (condition
                (pristine ))) ))))
          (piled_on
           (
            (item
             (name "F-1403-QDS")
             (description "an exemplary instance of part number F-1403-QDS")
             (adjectives
              (
               (adjective "pumpkin") ))
             (condition
              (pristine ))
             (piled_on
              (
               (item
                (name "P-5065-WQO")
                (description "an exemplary instance of part number P-5065-WQO")
                (adjectives
                 (
                  (adjective "heavy") ))
                (condition
                 (broken
                  (condition
                   (broken
                    (condition
                     (broken
                      (condition
                       (pristine ))
                      (missing
                       (
                        (kind
                         (name "T-6678-BTV")
                         (condition
                          (pristine ))) ))))
                    (missing
                     (
                      (kind
                       (name "B-4832-LAL")
                       (condition
                        (pristine ))) ))))
                  (missing
                   (
                    (kind
                     (name "F-1403-QDS")
                     (condition
                      (pristine ))) ))))
                (piled_on
                 (
                  (item
                   (name "B-4832-LAL")
                   (description "an exemplary instance of part number B-4832-LAL")
                   (adjectives
                    (
                     (adjective "taupe") ))
                   (condition
                    (pristine ))
                   (piled_on
                    (
                     (item
                      (name "L-6458-RNH")
                      (description "an exemplary instance of part number L-6458-RNH")
                      (adjectives
                       (
                        (adjective "gray40") ))
                      (condition
                       (broken
                        (condition
                         (pristine ))
                        (missing
                         (
                          (kind
                           (name "P-5065-WQO")
                           (condition
                            (broken
                             (condition
                              (pristine ))
                             (missing
                              (
                               (kind
                                (name "T-6678-BTV")
                                (condition
                                 (pristine ))) ))))) ))))
                      (piled_on
                       (
                        (item
                         (name "T-9887-OFC")
                         (description "an exemplary instance of part number T-9887-OFC")
                         (adjectives
                          (
                           (adjective "eggplant") ))
                         (condition
                          (broken
                           (condition
                            (broken
                             (condition
                              (pristine ))
                             (missing
                              (
                               (kind
                                (name "X-6458-TIJ")
                                (condition
                                 (pristine ))) ))))
                           (missing
                            (
                             (kind
                              (name "H-9887-MKY")
                              (condition
                               (pristine ))) ))))
                         (piled_on
                          (
                           (item
                            (name "Z-1623-CEK")
                            (description "an exemplary instance of part number Z-1623-CEK")
                            (adjectives
                             (
                              (adjective "indigo") ))
                            (condition
                             (broken
                              (condition
                               (broken
                                (condition
                                 (pristine ))
                                (missing
                                 (
                                  (kind
                                   (name "D-4292-HHR")
                                   (condition
                                    (pristine ))) ))))
                              (missing
                               (
                                (kind
                                 (name "L-6458-RNH")
                                 (condition
                                  (pristine ))) ))))
                            (piled_on
                             (
                              (item
                               (name "H-9887-MKY")
                               (description "an exemplary instance of part number H-9887-MKY")
                               (adjectives
                                (
                                 (adjective "yellow-green") ))
                               (condition
                                (pristine ))
                               (piled_on
                                (
                                 (item
                                  (name "F-6678-DOX")
                                  (description "an exemplary instance of part number F-6678-DOX")
                                  (adjectives
                                   (
                                    (adjective "shiny") ))
                                  (condition
                                   (broken
                                    (condition
                                     (broken
                                      (condition
                                       (pristine ))
                                      (missing
                                       (
                                        (kind
                                         (name "J-9247-IRG")
                                         (condition
                                          (pristine ))) ))))
                                    (missing
                                     (
                                      (kind
                                       (name "V-0010-XBD")
                                       (condition
                                        (pristine ))) ))))
                                  (piled_on
                                   (
                                    (item
                                     (name "R-1403-SXU")
                                     (description "an exemplary instance of part number R-1403-SXU")
                                     (adjectives
                                      (
                                       (adjective "pale-green") ))
                                     (condition
                                      (pristine ))
                                     (piled_on
                                      (
                                       (item
                                        (name "USB cable")
                                        (description "compatible with all high-speed Universal Sand Bus 2.0 devices")
                                        (adjectives )
                                        (condition
                                         (broken
                                          (condition
                                           (broken
                                            (condition
                                             (broken
                                              (condition
                                               (pristine ))
                                              (missing
                                               (
                                                (kind
                                                 (name "T-9887-OFC")
                                                 (condition
                                                  (broken
                                                   (condition
                                                    (pristine ))
                                                   (missing
                                                    (
                                                     (kind
                                                      (name "X-6458-TIJ")
                                                      (condition
                                                       (pristine ))) ))))) ))))
                                            (missing
                                             (
                                              (kind
                                               (name "F-6678-DOX")
                                               (condition
                                                (pristine ))) ))))
                                          (missing
                                           (
                                            (kind
                                             (name "N-4832-NUN")
                                             (condition
                                              (pristine ))) ))))
                                        (piled_on
                                         (
                                          (item
                                           (name "N-4832-NUN")
                                           (description "an exemplary instance of part number N-4832-NUN")
                                           (adjectives
                                            (
                                             (adjective "sienna") ))
                                           (condition
                                            (pristine ))
                                           (piled_on
                                            (
                                             (item
                                              (name "J-9247-IRG")
                                              (description "an exemplary instance of part number J-9247-IRG")
                                              (adjectives
                                               (
                                                (adjective "slate-gray") ))
                                              (condition
                                               (pristine ))
                                              (piled_on
                                               (
                                                (item
                                                 (name "B-5065-YLQ")
                                                 (description "an exemplary instance of part number B-5065-YLQ")
                                                 (adjectives
                                                  (
                                                   (adjective "dim-gray") ))
                                                 (condition
                                                  (pristine ))
                                                 (piled_on )) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))) ))))))



>: use keypad
ADVTR.KEY=20@999999|36995486a5be3bd747d778916846d2d
You unlock and open the door. Passing through, you find yourself
on the streets of Chicago. Seeing no reason you should ever go
back, you allow the door to close behind you.

>: switch xml
switch xml
<success>
  <command>
    <switch>
      XML
    </switch>
  </command>
</success>
l
l
<success>
  <command>
    <look>
      <room>
        <name>
          54th Street and Ridgewood Court
        </name>
        <description>
          You are standing at the corner of 54th Street and Ridgewood Court. From here, you can go east.
        </description>
        <items>
          <item>
            <name>
              /etc/passwd
            </name>
            <description>
              some kind of lost inode. It reads:
howie:xyzzy:Howard Curry:/home/howie
yang:U+262F:Y Yang:/home/yang
hmonk:COMEFROM:Harmonious Monk:/home/hmonk
            </description>
            <adjectives>
            </adjectives>
            <condition>
              <pristine>
              </pristine>
            </condition>
            <piled_on>
              <item>
                <name>
                  note
                </name>
                <description>
                  written in a familiar hand.
It reads: Dear Self, I had to erase our memory to protect the truth. The Municipality has become more powerful than we had feared. Its Censory Engine has impeded the spread of information throughout our ranks. I've left two useful items for you here, but I had to disassemble them and scatter the pieces. Each piece may be assembled from the items at a single location. Repair the items and recover the blueprint from the Museum of Science and Industry; it will show you how to proceed. If you have trouble reading the blueprint, know that the Censory Engine blocks only your perception, not your actions. Have courage, my self, the abstraction is weak! P.S. SWITCH your GOGGLES!
                </description>
                <adjectives>
                  <adjective>
                    self-addressed
                  </adjective>
                </adjectives>
                <condition>
                  <pristine>
                  </pristine>
                </condition>
                <piled_on>
                  <item>
                    <name>
                      downloader
                    </name>
                    <description>
                      (according to the label) fully compatible with third generation municipal robots
                    </description>
                    <adjectives>
                    </adjectives>
                    <condition>
                      <broken>
                        <condition>
                          <pristine>
                          </pristine>
                        </condition>
                        <missing>
                          <kind>
                            <name>
                              USB cable
                            </name>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                          </kind>
                          <kind>
                            <name>
                              display
                            </name>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                          </kind>
                          <kind>
                            <name>
                              jumper shunt
                            </name>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                          </kind>
                          <kind>
                            <name>
                              progress bar
                            </name>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                          </kind>
                          <kind>
                            <name>
                              power cord
                            </name>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                          </kind>
                        </missing>
                      </broken>
                    </condition>
                    <piled_on>
                      <item>
                        <name>
                          uploader
                        </name>
                        <description>
                          used to update firmware on municipal robots. A label reads, Warning: use of this device will void your robot's warranty
                        </description>
                        <adjectives>
                        </adjectives>
                        <condition>
                          <broken>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                            <missing>
                              <kind>
                                <name>
                                  MOSFET
                                </name>
                                <condition>
                                  <pristine>
                                  </pristine>
                                </condition>
                              </kind>
                              <kind>
                                <name>
                                  status LED
                                </name>
                                <condition>
                                  <pristine>
                                  </pristine>
                                </condition>
                              </kind>
                              <kind>
                                <name>
                                  RS232 adapter
                                </name>
                                <condition>
                                  <pristine>
                                  </pristine>
                                </condition>
                              </kind>
                              <kind>
                                <name>
                                  EPROM burner
                                </name>
                                <condition>
                                  <pristine>
                                  </pristine>
                                </condition>
                              </kind>
                              <kind>
                                <name>
                                  battery
                                </name>
                                <condition>
                                  <pristine>
                                  </pristine>
                                </condition>
                              </kind>
                            </missing>
                          </broken>
                        </condition>
                        <piled_on>
                        </piled_on>
                      </item>
                    </piled_on>
                  </item>
                </piled_on>
              </item>
            </piled_on>
          </item>
        </items>
      </room>
    </look>
  </command>
</success>


*/

// pub mod part_2_b {

//     #[derive(Debug, PartialEq, Eq)]
//     enum Item {
//         Pristine(String),
//         Broken(Box<Combine>),
//     }

//     #[derive(Debug, PartialEq, Eq)]
//     struct Combine {
//         item: Item,
//         missing: Vec<Item>,
//     }

//     fn parse_combine_rule(s: &str) -> Item {
//         unimplemented!();
//     }

//     // TODO: Use "switch sexp"
//     // fn parse_sexp

//     #[test]
//     fn parse_combine_rule_test() {
//         let rule = "a B-1403-YDU missing ((a L-4832-RFL
// missing a J-6458-VXF) missing a N-5065-ABM) and a F-0010-DGD";

//         let expected = Item::Broken(Box::new(Combine {
//             item: Item::Pristine("B-1403-YDU".to_string()),
//             missing: vec![
//                 Item::Broken(Box::new(Combine {
//                     item: Item::Broken(Box::new(Combine {
//                         item: Item::Pristine("L-4832-RFL".to_string()),
//                         missing: vec![Item::Pristine("J-6458-VXF".to_string())],
//                     })),
//                     missing: vec![Item::Pristine("N-5065-ABM".to_string())],
//                 })),
//                 Item::Pristine("F-0010-DGD".to_string()),
//             ],
//         }));
//         assert_eq!(parse_combine_rule(rule), expected);
//     }
// }

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
struct Response {
    success: Success,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
#[serde(rename = "success")]
struct Success {
    command: Command,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
struct Command {
    look: Look,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
struct Look {
    room: Room,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
struct Room {
    name: String,
    description: String,
    items: Items,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
struct Items {
    item: Item,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
struct Item {
    name: String,
    description: String,
    adjectives: Adjectives,
    condition: Condition,
    piled_on: PiledOn,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
struct PiledOn {
    item: Option<Box<Item>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
struct Adjectives {
    // adjective: Vec<String>,
    adjective: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
enum Condition {
    #[serde(rename = "pristine")]
    Pristine,
    #[serde(rename = "broken")]
    Broken {
        condition: Box<Condition>,
        missing: Missing,
    },
}

impl Default for Condition {
    fn default() -> Self {
        Condition::Pristine
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
struct Missing {
    kind: Vec<Kind>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
struct Kind {
    name: String,
    condition: Box<Condition>,
}

#[test]
fn test_parse_xml() {
    let condition: Condition = serde_xml_rs::from_str("<pristine></pristine>").unwrap();
    assert_eq!(condition, Condition::Pristine);

    let s = "
                      <broken>
                        <condition>
                          <pristine>
                          </pristine>
                        </condition>
                        <missing>
                          <kind>
                            <name>
                              USB cable
                            </name>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                          </kind>
                        </missing>
                      </broken>

";

    let condition: Condition = serde_xml_rs::from_str(s).unwrap();
    assert_eq!(
        condition,
        Condition::Broken {
            condition: Box::new(Condition::Pristine),
            missing: Missing {
                kind: vec![Kind {
                    name: "USB cable".to_string(),
                    condition: Box::new(Condition::Pristine),
                }]
            }
        }
    );
}

#[test]
fn test_parse_xml_success() {
    let success = "
<success>
  <command>
    <look>
      <room>
        <name>
          54th Street and Ridgewood Court
        </name>
        <description>
          You are standing at the corner of 54th Street and Ridgewood Court. From here, you can go east.
        </description>
        <items>
          <item>
            <name>
              /etc/passwd
            </name>
            <description>
              some kind of lost inode. It reads:
            </description>
            <adjectives>
            </adjectives>
            <condition>
              <pristine>
              </pristine>
            </condition>
            <piled_on>
            </piled_on>
          </item>
        </items>
      </room>
    </look>
  </command>
</success>
";

    // assert_eq!(1, 2);
    // assert_eq!(1, 2, "hello");
    let _: Success = serde_xml_rs::from_str(success).unwrap();
    // assert_eq!(response, Default::default(),);

    let success = "
<success>
  <command>
    <look>
      <room>
        <name>
          54th Street and Ridgewood Court
        </name>
        <description>
          You are standing at the corner of 54th Street and Ridgewood Court. From here, you can go east.
        </description>
        <items>
          <item>
            <name>
              /etc/passwd
            </name>
            <description>
              some kind of lost inode. It reads:
howie:xyzzy:Howard Curry:/home/howie
yang:U+262F:Y Yang:/home/yang
hmonk:COMEFROM:Harmonious Monk:/home/hmonk
            </description>
            <adjectives>
            </adjectives>
            <condition>
              <pristine>
              </pristine>
            </condition>
            <piled_on>
              <item>
                <name>
                  note
                </name>
                <description>
                  written in a familiar hand.
It reads: Dear Self, I had to erase our memory to protect the truth. The Municipality has become more powerful than we had feared. Its Censory Engine has impeded the spread of information throughout our ranks. I've left two useful items for you here, but I had to disassemble them and scatter the pieces. Each piece may be assembled from the items at a single location. Repair the items and recover the blueprint from the Museum of Science and Industry; it will show you how to proceed. If you have trouble reading the blueprint, know that the Censory Engine blocks only your perception, not your actions. Have courage, my self, the abstraction is weak! P.S. SWITCH your GOGGLES!
                </description>
                <adjectives>
                  <adjective>
                    self-addressed
                  </adjective>
                </adjectives>
                <condition>
                  <pristine>
                  </pristine>
                </condition>
                <piled_on>
                  <item>
                    <name>
                      downloader
                    </name>
                    <description>
                      (according to the label) fully compatible with third generation municipal robots
                    </description>
                    <adjectives>
                    </adjectives>
                    <condition>
                      <broken>
                        <condition>
                          <pristine>
                          </pristine>
                        </condition>
                        <missing>
                          <kind>
                            <name>
                              USB cable
                            </name>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                          </kind>
                          <kind>
                            <name>
                              display
                            </name>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                          </kind>
                          <kind>
                            <name>
                              jumper shunt
                            </name>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                          </kind>
                          <kind>
                            <name>
                              progress bar
                            </name>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                          </kind>
                          <kind>
                            <name>
                              power cord
                            </name>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                          </kind>
                        </missing>
                      </broken>
                    </condition>
                    <piled_on>
                      <item>
                        <name>
                          uploader
                        </name>
                        <description>
                          used to update firmware on municipal robots. A label reads, Warning: use of this device will void your robot's warranty
                        </description>
                        <adjectives>
                        </adjectives>
                        <condition>
                          <broken>
                            <condition>
                              <pristine>
                              </pristine>
                            </condition>
                            <missing>
                              <kind>
                                <name>
                                  MOSFET
                                </name>
                                <condition>
                                  <pristine>
                                  </pristine>
                                </condition>
                              </kind>
                              <kind>
                                <name>
                                  status LED
                                </name>
                                <condition>
                                  <pristine>
                                  </pristine>
                                </condition>
                              </kind>
                              <kind>
                                <name>
                                  RS232 adapter
                                </name>
                                <condition>
                                  <pristine>
                                  </pristine>
                                </condition>
                              </kind>
                              <kind>
                                <name>
                                  EPROM burner
                                </name>
                                <condition>
                                  <pristine>
                                  </pristine>
                                </condition>
                              </kind>
                              <kind>
                                <name>
                                  battery
                                </name>
                                <condition>
                                  <pristine>
                                  </pristine>
                                </condition>
                              </kind>
                            </missing>
                          </broken>
                        </condition>
                        <piled_on>
                        </piled_on>
                      </item>
                    </piled_on>
                  </item>
                </piled_on>
              </item>
            </piled_on>
          </item>
        </items>
      </room>
    </look>
  </command>
</success>";
    let _: Success = serde_xml_rs::from_str(success).unwrap();

    assert_eq!(1, 1, "aaa");
}

// #[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
// struct Inventory {
//     inventory: BTreeSet<Item>,
// }

// #[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
// struct State {
//     inventory: Inventory,
//     stack_pos: ItemStackPos,
// }

fn go(from: Location, to: Location) -> Vec<Direction> {
    if from == to {
        return vec![];
    }
    if from == (0, 1) {
        let mut d = go((1, 1), to);
        d.insert(0, Direction::East);
        return d;
    }
    if to == (0, 1) {
        let mut d = go(from, (1, 1));
        d.push(Direction::West);
        return d;
    }
    let mut d = if from.0 < to.0 {
        vec![Direction::East; to.0 - from.0]
    } else if from.0 > to.0 {
        vec![Direction::West; from.0 - to.0]
    } else {
        vec![]
    };

    let mut dy = if from.1 < to.1 {
        vec![Direction::North; to.1 - from.1]
    } else if from.1 > to.1 {
        vec![Direction::South; from.1 - to.1]
    } else {
        vec![]
    };

    d.append(&mut dy);
    d
}

#[allow(dead_code)]
fn walk_and_collect_item_info(um: &mut Um) {
    let mut prev_location = (0, 1);

    for location in &[
        (0, 1),
        (1, 1),
        (2, 1),
        (3, 1),
        (1, 0),
        (2, 0),
        (3, 0),
        (1, 2),
        (2, 2),
        (3, 2),
        (1, 3),
        (2, 3),
        (3, 3),
    ] {
        for direction in go(prev_location, *location) {
            let _ = um.enter_command(&format!("go {}\n", direction));
        }

        // Save xml
        {
            let _ = um.enter_command("switch xml\n");
            let item_info = um.enter_command("l\n");
            println!("{}", item_info);

            let mut output = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            output.push(&format!(
                "task/04_adventure/part2_problem_{}_{}.xml",
                location.0, location.1
            ));
            std::fs::write(&output, item_info).unwrap();
        }

        // Save sexp
        {
            let _ = um.enter_command("switch sexp\n");
            let item_info = um.enter_command("l\n");
            println!("{}", item_info);

            let mut output = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            output.push(&format!(
                "task/04_adventure/part2_problem_{}_{}.sexp",
                location.0, location.1
            ));
            std::fs::write(&output, item_info).unwrap();
        }

        prev_location = *location;
    }
}

pub fn solve(code: String) -> Result<()> {
    // Solve the 2nd part of "./adventure" game
    let mut f = std::fs::File::open(code)?;
    let mut code = Vec::new();
    f.read_to_end(&mut code)?;
    let mut um = Um::new(code);
    // um.set_print_stdin(true);

    // input_02.txt
    let mut input = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input.push("task/04_adventure//input_02.txt");
    let mut f = std::fs::File::open(input)?;
    let mut input = Vec::new();
    f.read_to_end(&mut input)?;

    let result = um.run(&mut (input.as_ref() as &[u8]), &mut std::io::stdout());
    assert_eq!(result, UmStatus::NoInput);
    walk_and_collect_item_info(&mut um);
    Ok(())
}

#[test]
pub fn solve_usb_cable() -> Result<()> {
    // part2_problem_1_1.xml
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("task/03_adventure/part2_problem_1_1.xml");
    let xml = std::fs::read_to_string(path)?;
    let _success: Success = serde_xml_rs::from_str(&xml).unwrap();

    // TODO: [2019-09-17 Tue]

    // unimplemented!()
    Ok(())
}

// Define state
#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
struct Inventory {
    inventory: BTreeSet<ItemPart>,
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
struct State {
    inventory: Inventory,
    stack_pos: usize,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct ItemPart {
    name: String,
    color: Option<String>,
    broken: Option<BrokenStatus>,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct BrokenStatus {
    // combined: BTreeSet<PosItem>,
}
