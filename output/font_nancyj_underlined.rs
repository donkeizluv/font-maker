
pub const HEIGHT: usize = 8;

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
   
   
   
   
   
   
ooo
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
ooooooooo
         "#,
r#"
d88  
 88  
 88  
 88  
 88  
d88P 
ooooo
     "#,
r#"
d8888b. 
    `88 
.aaadP' 
88'     
88.     
Y88888P 
oooooooo
        "#,
r#"
d8888b. 
    `88 
 aaad8' 
    `88 
    .88 
d88888P 
oooooooo
        "#,
r#"
dP   dP 
88   88 
88aaa88 
     88 
     88 
     dP 
oooooooo
        "#,
r#"
888888P 
88'     
88baaa. 
    `88 
     88 
d88888P 
oooooooo
        "#,
r#"
.d8888P 
88'     
88baaa. 
88` `88 
8b. .d8 
`Y888P' 
oooooooo
        "#,
r#"
d88888P 
    d8' 
   d8'  
  d8'   
 d8'    
d8'     
oooooooo
        "#,
r#"
.d888b. 
Y8' `8P 
d8bad8b 
88` `88 
8b. .88 
Y88888P 
oooooooo
        "#,
r#"
.d888b. 
Y8' `88 
`8bad88 
    `88 
d.  .88 
`8888P  
oooooooo
        "#,
"",
"",
"",
"",
"",
"",
"",
r#"
 .d888888  
d8'    88  
88aaaaa88a 
88     88  
88     88  
88     88  
ooooooooooo
           "#,
r#"
 888888ba  
 88    `8b 
a88aaaa8P' 
 88   `8b. 
 88    .88 
 88888888P 
ooooooooooo
           "#,
r#"
 a88888b. 
d8'   `88 
88        
88        
Y8.   .88 
 Y88888P' 
oooooooooo
          "#,
r#"
888888ba  
88    `8b 
88     88 
88     88 
88    .8P 
8888888P  
oooooooooo
          "#,
r#"
 88888888b 
 88        
a88aaaa    
 88        
 88        
 88888888P 
ooooooooooo
           "#,
r#"
 88888888b 
 88        
a88aaaa    
 88        
 88        
 dP        
ooooooooooo
           "#,
r#"
 .88888.  
d8'   `88 
88        
88   YP88 
Y8.   .88 
 `88888'  
oooooooooo
          "#,
r#"
dP     dP  
88     88  
88aaaaa88a 
88     88  
88     88  
dP     dP  
ooooooooooo
           "#,
r#"
dP 
88 
88 
88 
88 
dP 
ooo
   "#,
r#"
       dP 
       88 
       88 
       88 
88.  .d8P 
 `Y8888'  
oooooooooo
          "#,
r#"
dP     dP 
88   .d8' 
88aaa8P'  
88   `8b. 
88     88 
dP     dP 
oooooooooo
          "#,
r#"
dP        
88        
88        
88        
88        
88888888P 
oooooooooo
          "#,
r#"
8888ba.88ba  
88  `8b  `8b 
88   88   88 
88   88   88 
88   88   88 
dP   dP   dP 
ooooooooooooo
             "#,
r#"
888888ba  
88    `8b 
88     88 
88     88 
88     88 
dP     dP 
oooooooooo
          "#,
r#"
 .88888.  
d8'   `8b 
88     88 
88     88 
Y8.   .8P 
 `8888P'  
oooooooooo
          "#,
r#"
 888888ba  
 88    `8b 
a88aaaa8P' 
 88        
 88        
 dP        
ooooooooooo
           "#,
r#"
 .88888.   
d8'   `8b  
88     88  
88  db 88  
Y8.  Y88P  
 `8888PY8b 
ooooooooooo
           "#,
r#"
 888888ba  
 88    `8b 
a88aaaa8P' 
 88   `8b. 
 88     88 
 dP     dP 
ooooooooooo
           "#,
r#"
.d88888b  
88.    "' 
`Y88888b. 
      `8b 
d8'   .8P 
 Y88888P  
oooooooooo
          "#,
r#"
d888888P 
   88    
   88    
   88    
   88    
   dP    
ooooooooo
         "#,
r#"
dP     dP 
88     88 
88     88 
88     88 
Y8.   .8P 
`Y88888P' 
oooooooooo
          "#,
r#"
dP     dP 
88     88 
88    .8P 
88    d8' 
88  .d8P  
888888'   
oooooooooo
          "#,
r#"
dP   dP   dP 
88   88   88 
88  .8P  .8P 
88  d8'  d8' 
88.d8P8.d8P  
8888' Y88'   
ooooooooooooo
             "#,
r#"
dP    dP 
Y8.  .8P 
 Y8aa8P  
d8'  `8b 
88    88 
dP    dP 
ooooooooo
         "#,
r#"
dP    dP 
Y8.  .8P 
 Y8aa8P  
   88    
   88    
   dP    
ooooooooo
         "#,
r#"
d8888888P 
     .d8' 
   .d8'   
 .d8'     
d8'       
Y8888888P 
oooooooooo
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
ooooooooo
         "#,
r#"
dP       
88       
88d888b. 
88'  `88 
88.  .88 
88Y8888' 
ooooooooo
         "#,
r#"
         
         
.d8888b. 
88'  `"" 
88.  ... 
`88888P' 
ooooooooo
         "#,
r#"
      dP 
      88 
.d888b88 
88'  `88 
88.  .88 
`88888P8 
ooooooooo
         "#,
r#"
         
         
.d8888b. 
88ooood8 
88.  ... 
`88888P' 
ooooooooo
         "#,
r#"
.8888b 
88   " 
88aaa  
88     
88     
dP     
ooooooo
       "#,
r#"
         
         
.d8888b. 
88'  `88 
88.  .88 
`8888P88 
o~~~~.88~
 d8888P  "#,
r#"
dP       
88       
88d888b. 
88'  `88 
88    88 
dP    dP 
ooooooooo
         "#,
r#"
oo 
   
dP 
88 
88 
dP 
ooo
   "#,
r#"
oo 
   
dP 
88 
88 
88 
88~
dP "#,
r#"
dP       
88       
88  .dP  
88888"   
88  `8b. 
dP   `YP 
ooooooooo
         "#,
r#"
dP 
88 
88 
88 
88 
dP 
ooo
   "#,
r#"
           
           
88d8b.d8b. 
88'`88'`88 
88  88  88 
dP  dP  dP 
ooooooooooo
           "#,
r#"
         
         
88d888b. 
88'  `88 
88    88 
dP    dP 
ooooooooo
         "#,
r#"
         
         
.d8888b. 
88'  `88 
88.  .88 
`88888P' 
ooooooooo
         "#,
r#"
          
          
 88d888b. 
 88'  `88 
 88.  .88 
 88Y888P' 
~88~oooooo
 dP       "#,
r#"
         
         
.d8888b. 
88'  `88 
88.  .88 
`8888P88 
ooooo~88~
      dP "#,
r#"
         
         
88d888b. 
88'  `88 
88       
dP       
ooooooooo
         "#,
r#"
         
         
.d8888b. 
Y8ooooo. 
      88 
`88888P' 
ooooooooo
         "#,
r#"
  dP   
  88   
d8888P 
  88   
  88   
  dP   
ooooooo
       "#,
r#"
         
         
dP    dP 
88    88 
88.  .88 
`88888P' 
ooooooooo
         "#,
r#"
         
         
dP   .dP 
88   d8' 
88 .88'  
8888P'   
ooooooooo
         "#,
r#"
           
           
dP  dP  dP 
88  88  88 
88.88b.88' 
8888P Y8P  
ooooooooooo
           "#,
r#"
         
         
dP.  .dP 
 `8bd8'  
 .d88b.  
dP'  `dP 
ooooooooo
         "#,
r#"
         
         
dP    dP 
88    88 
88.  .88 
`8888P88 
o~~~~.88~
 d8888P  "#,
r#"
         
         
d888888b 
   .d8P' 
 .Y8P    
d888888P 
ooooooooo
         "#,
"",
"",
"",
"",
];    
