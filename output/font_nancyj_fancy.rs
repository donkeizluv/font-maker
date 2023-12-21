
pub const HEIGHT: u8 = 8;

pub const SET: [&str; 127] = [
    
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
r#"
   
   
   
   
   
   
   
   "#,
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
"",
r#"
 a8888a  
d8' ..8b 
88 .P 88 
88 d' 88 
Y8'' .8P 
 Y8888P  
         
         "#,
r#"
d88  
 88  
 88  
 88  
 88  
d88P 
     
     "#,
r#"
d8888b. 
    `88 
.aaadP' 
88'     
88.     
Y88888P 
        
        "#,
r#"
d8888b. 
    `88 
 aaad8' 
    `88 
    .88 
d88888P 
        
        "#,
r#"
dP   dP 
88   88 
88aaa88 
     88 
     88 
     dP 
        
        "#,
r#"
888888P 
88'     
88baaa. 
    `88 
     88 
d88888P 
        
        "#,
r#"
.d8888P 
88'     
88baaa. 
88` `88 
8b. .d8 
`Y888P' 
        
        "#,
r#"
d88888P 
    d8' 
   d8'  
  d8'   
 d8'    
d8'     
        
        "#,
r#"
.d888b. 
Y8' `8P 
d8bad8b 
88` `88 
8b. .88 
Y88888P 
        
        "#,
r#"
.d888b. 
Y8' `88 
`8bad88 
    `88 
d.  .88 
`8888P  
        
        "#,
"",
"",
"",
"",
"",
"",
"",
r#"
MMP"""""""MM 
M' .mmmm  MM 
M         `M 
M  MMMMM  MM 
M  MMMMM  MM 
M  MMMMM  MM 
MMMMMMMMMMMM 
             "#,
r#"
M#"""""""'M  
##  mmmm. `M 
#'        .M 
M#  MMMb.'YM 
M#  MMMM'  M 
M#       .;M 
M#########M  
             "#,
r#"
MM'""""'YMM 
M' .mmm. `M 
M  MMMMMooM 
M  MMMMMMMM 
M. `MMM' .M 
MM.     .dM 
MMMMMMMMMMM 
            "#,
r#"
M""""""'YMM 
M  mmmm. `M 
M  MMMMM  M 
M  MMMMM  M 
M  MMMM' .M 
M       .MM 
MMMMMMMMMMM 
            "#,
r#"
MM""""""""`M 
MM  mmmmmmmM 
M`      MMMM 
MM  MMMMMMMM 
MM  MMMMMMMM 
MM        .M 
MMMMMMMMMMMM 
             "#,
r#"
MM""""""""`M 
MM  mmmmmmmM 
M'      MMMM 
MM  MMMMMMMM 
MM  MMMMMMMM 
MM  MMMMMMMM 
MMMMMMMMMMMM 
             "#,
r#"
MM'"""""`MM 
M' .mmm. `M 
M  MMMMMMMM 
M  MMM   `M 
M. `MMM' .M 
MM.     .MM 
MMMMMMMMMMM 
            "#,
r#"
M""MMMMM""MM 
M  MMMMM  MM 
M         `M 
M  MMMMM  MM 
M  MMMMM  MM 
M  MMMMM  MM 
MMMMMMMMMMMM 
             "#,
r#"
M""M 
M  M 
M  M 
M  M 
M  M 
M  M 
MMMM 
     "#,
r#"
MMMMMMMM""M 
MMMMMMMM  M 
MMMMMMMM  M 
MMMMMMMM  M 
M. `MMM' .M 
MM.     .MM 
MMMMMMMMMMM 
            "#,
r#"
M""MMMMM""M 
M  MMMM' .M 
M       .MM 
M  MMMb. YM 
M  MMMMb  M 
M  MMMMM  M 
MMMMMMMMMMM 
            "#,
r#"
M""MMMMMMMM 
M  MMMMMMMM 
M  MMMMMMMM 
M  MMMMMMMM 
M  MMMMMMMM 
M         M 
MMMMMMMMMMM 
            "#,
r#"
M"""""`'"""`YM 
M  mm.  mm.  M 
M  MMM  MMM  M 
M  MMM  MMM  M 
M  MMM  MMM  M 
M  MMM  MMM  M 
MMMMMMMMMMMMMM 
               "#,
r#"
M"""""""`YM 
M  mmmm.  M 
M  MMMMM  M 
M  MMMMM  M 
M  MMMMM  M 
M  MMMMM  M 
MMMMMMMMMMM 
            "#,
r#"
MMP"""""YMM 
M' .mmm. `M 
M  MMMMM  M 
M  MMMMM  M 
M. `MMM' .M 
MMb     dMM 
MMMMMMMMMMM 
            "#,
r#"
MM"""""""`YM 
MM  mmmmm  M 
M'        .M 
MM  MMMMMMMM 
MM  MMMMMMMM 
MM  MMMMMMMM 
MMMMMMMMMMMM 
             "#,
r#"
MM'"""""`MMM 
M  .mmm,  MM 
M  MMMMM  MM 
M  MM  M  MM 
M  `MM    MM 
MM.    .. `M 
MMMMMMMMMMMM 
             "#,
r#"
MM"""""""`MM 
MM  mmmm,  M 
M'        .M 
MM  MMMb. "M 
MM  MMMMM  M 
MM  MMMMM  M 
MMMMMMMMMMMM 
             "#,
r#"
MP""""""`MM 
M  mmmmm..M 
M.      `YM 
MMMMMMM.  M 
M. .MMM'  M 
Mb.     .dM 
MMMMMMMMMMM 
            "#,
r#"
M""""""""M 
Mmmm  mmmM 
MMMM  MMMM 
MMMM  MMMM 
MMMM  MMMM 
MMMM  MMMM 
MMMMMMMMMM 
           "#,
r#"
M""MMMMM""M 
M  MMMMM  M 
M  MMMMM  M 
M  MMMMM  M 
M  `MMM'  M 
Mb       dM 
MMMMMMMMMMM 
            "#,
r#"
M""MMMMM""M 
M  MMMMM  M 
M  MMMMP  M 
M  MMMM' .M 
M  MMP' .MM 
M     .dMMM 
MMMMMMMMMMM 
            "#,
r#"
M""MMM""MMM""M 
M  MMM  MMM  M 
M  MMP  MMP  M 
M  MM'  MM' .M 
M  `' . '' .MM 
M    .d  .dMMM 
MMMMMMMMMMMMMM 
               "#,
r#"
M""MMMM""M 
M  `MM'  M 
MM.    .MM 
M  .mm.  M 
M  MMMM  M 
M  MMMM  M 
MMMMMMMMMM 
           "#,
r#"
M""MMMM""M 
M. `MM' .M 
MM.    .MM 
MMMb  dMMM 
MMMM  MMMM 
MMMM  MMMM 
MMMMMMMMMM 
           "#,
r#"
M""""""""`M 
Mmmmmm   .M 
MMMMP  .MMM 
MMP  .MMMMM 
M' .MMMMMMM 
M         M 
MMMMMMMMMMM 
            "#,
"",
"",
"",
"",
"",
"",
r#"
         
         
.d8888b. 
88'  `88 
88.  .88 
`88888P8 
         
         "#,
r#"
dP       
88       
88d888b. 
88'  `88 
88.  .88 
88Y8888' 
         
         "#,
r#"
         
         
.d8888b. 
88'  `"" 
88.  ... 
`88888P' 
         
         "#,
r#"
      dP 
      88 
.d888b88 
88'  `88 
88.  .88 
`88888P8 
         
         "#,
r#"
         
         
.d8888b. 
88ooood8 
88.  ... 
`88888P' 
         
         "#,
r#"
.8888b 
88   " 
88aaa  
88     
88     
dP     
       
       "#,
r#"
         
         
.d8888b. 
88'  `88 
88.  .88 
`8888P88 
     .88 
 d8888P  "#,
r#"
dP       
88       
88d888b. 
88'  `88 
88    88 
dP    dP 
         
         "#,
r#"
oo 
   
dP 
88 
88 
dP 
   
   "#,
r#"
oo 
   
dP 
88 
88 
88 
88 
dP "#,
r#"
dP       
88       
88  .dP  
88888"   
88  `8b. 
dP   `YP 
         
         "#,
r#"
dP 
88 
88 
88 
88 
dP 
   
   "#,
r#"
           
           
88d8b.d8b. 
88'`88'`88 
88  88  88 
dP  dP  dP 
           
           "#,
r#"
         
         
88d888b. 
88'  `88 
88    88 
dP    dP 
         
         "#,
r#"
         
         
.d8888b. 
88'  `88 
88.  .88 
`88888P' 
         
         "#,
r#"
         
         
88d888b. 
88'  `88 
88.  .88 
88Y888P' 
88       
dP       "#,
r#"
         
         
.d8888b. 
88'  `88 
88.  .88 
`8888P88 
      88 
      dP "#,
r#"
         
         
88d888b. 
88'  `88 
88       
dP       
         
         "#,
r#"
         
         
.d8888b. 
Y8ooooo. 
      88 
`88888P' 
         
         "#,
r#"
  dP   
  88   
d8888P 
  88   
  88   
  dP   
       
       "#,
r#"
         
         
dP    dP 
88    88 
88.  .88 
`88888P' 
         
         "#,
r#"
         
         
dP   .dP 
88   d8' 
88 .88'  
8888P'   
         
         "#,
r#"
           
           
dP  dP  dP 
88  88  88 
88.88b.88' 
8888P Y8P  
           
           "#,
r#"
         
         
dP.  .dP 
 `8bd8'  
 .d88b.  
dP'  `dP 
         
         "#,
r#"
         
         
dP    dP 
88    88 
88.  .88 
`8888P88 
     .88 
 d8888P  "#,
r#"
         
         
d888888b 
   .d8P' 
 .Y8P    
d888888P 
         
         "#,
"",
"",
"",
"",
];    
