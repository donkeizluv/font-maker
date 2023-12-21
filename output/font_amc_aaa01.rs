
pub const HEIGHT: u8 = 15;

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
 .S_SSSs    
.SS~SSSSS   
S%S   SSSS  
S%S    S%S  
S%S SSSS%S  
S&S  SSS%S  
S&S    S&S  
S&S    S&S  
S*S    S&S  
S*S    S*S  
S*S    S*S  
SSS    S*S  
       SP   
       Y    
            "#,
r#"
 .S_SSSs    
.SS~SSSSS   
S%S   SSSS  
S%S    S%S  
S%S SSSS%P  
S&S  SSSY   
S&S    S&S  
S&S    S&S  
S*S    S&S  
S*S    S*S  
S*S SSSSP   
S*S  SSY    
SP          
Y           
            "#,
r#"
  sSSs  
 d%%SP  
d%S'    
S%S     
S&S     
S&S     
S&S     
S&S     
S*b     
S*S.    
 SSSbs  
  YSSP  
        
        
        "#,
r#"
 .S_sSSs    
.SS~YS%%b   
S%S   `S%b  
S%S    S%S  
S%S    S&S  
S&S    S&S  
S&S    S&S  
S&S    S&S  
S*S    d*S  
S*S   .S*S  
S*S_sdSSS   
SSS~YSSY    
            
            
            "#,
r#"
  sSSs  
 d%%SP  
d%S'    
S%S     
S&S     
S&S_Ss  
S&S~SP  
S&S     
S*b     
S*S.    
 SSSbs  
  YSSP  
        
        
        "#,
r#"
  sSSs  
 d%%SP  
d%S'    
S%S     
S&S     
S&S_Ss  
S&S~SP  
S&S     
S*b     
S*S     
S*S     
S*S     
SP      
Y       
        "#,
r#"
  sSSSSs  
 d%%%%SP  
d%S'      
S%S       
S&S       
S&S       
S&S       
S&S sSSs  
S*b `S%%  
S*S   S%  
 SS_sSSS  
  Y~YSSY  
          
          
          "#,
r#"
 .S    S.   
.SS    SS.  
S%S    S%S  
S%S    S%S  
S%S SSSS%S  
S&S  SSS&S  
S&S    S&S  
S&S    S&S  
S*S    S*S  
S*S    S*S  
S*S    S*S  
SSS    S*S  
       SP   
       Y    
            "#,
r#"
 .S  
.SS  
S%S  
S%S  
S&S  
S&S  
S&S  
S&S  
S*S  
S*S  
S*S  
S*S  
SP   
Y    
     "#,
r#"
    .S  
   .SS  
   S%S  
   S%S  
   S&S  
   S&S  
   S&S  
   S&S  
   d*S  
  .S*S  
sdSSS   
YSSY    
        
        
        "#,
r#"
 .S    S.   
.SS    SS.  
S%S    S&S  
S%S    d*S  
S&S   .S*S  
S&S_sdSSS   
S&S~YSSY%b  
S&S    `S%  
S*S     S%  
S*S     S&  
S*S     S&  
S*S     SS  
SP          
Y           
            "#,
r#"
S.      
SS.     
S%S     
S%S     
S&S     
S&S     
S&S     
S&S     
S*b     
S*S.    
 SSSbs  
  YSSP  
        
        
        "#,
r#"
 .S_SsS_S.   
.SS~S*S~SS.  
S%S `Y' S%S  
S%S     S%S  
S%S     S%S  
S&S     S&S  
S&S     S&S  
S&S     S&S  
S*S     S*S  
S*S     S*S  
S*S     S*S  
SSS     S*S  
        SP   
        Y    
             "#,
r#"
 .S_sSSs    
.SS~YS%%b   
S%S   `S%b  
S%S    S%S  
S%S    S&S  
S&S    S&S  
S&S    S&S  
S&S    S&S  
S*S    S*S  
S*S    S*S  
S*S    S*S  
S*S    SSS  
SP          
Y           
            "#,
r#"
  sSSs_sSSs    
 d%%SP~YS%%b   
d%S'     `S%b  
S%S       S%S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S*b       d*S  
S*S.     .S*S  
 SSSbs_sdSSS   
  YSSP~YSSY    
               
               
               "#,
r#"
 .S_sSSs    
.SS~YS%%b   
S%S   `S%b  
S%S    S%S  
S%S    d*S  
S&S   .S*S  
S&S_sdSSS   
S&S~YSSY    
S*S         
S*S         
S*S         
S*S         
SP          
Y           
            "#,
r#"
  sSSs_sSSs    
 d%%SP~YS%%b   
d%S'     `S%b  
S%S       S%S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S*b       d*S  
S*S.     .S*S  
 SSSbs_sdSSSS  
  YSSP~YSSSSS  
               
               
               "#,
r#"
 .S_sSSs    
.SS~YS%%b   
S%S   `S%b  
S%S    S%S  
S%S    d*S  
S&S   .S*S  
S&S_sdSSS   
S&S~YSY%b   
S*S   `S%b  
S*S    S%S  
S*S    S&S  
S*S    SSS  
SP          
Y           
            "#,
r#"
  sSSs  
 d%%SP  
d%S'    
S%|     
S&S     
Y&Ss    
`S&&S   
  `S*S  
   l*S  
  .S*P  
sSS*S   
YSS'    
        
        
        "#,
r#"
sdSS_SSSSSSbs  
YSSS~S%SSSSSP  
     S%S       
     S%S       
     S&S       
     S&S       
     S&S       
     S&S       
     S*S       
     S*S       
     S*S       
     S*S       
     SP        
     Y         
               "#,
r#"
 .S       S.   
.SS       SS.  
S%S       S%S  
S%S       S%S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S*b       d*S  
S*S.     .S*S  
 SSSbs_sdSSS   
  YSSP~YSSY    
               
               
               "#,
r#"
 .S    S.   
.SS    SS.  
S%S    S%S  
S%S    S%S  
S&S    S%S  
S&S    S&S  
S&S    S&S  
S&S    S&S  
S*b    S*S  
S*S.   S*S  
 SSSbs_S*S  
  YSSP~SSS  
            
            
            "#,
r#"
 .S     S.   
.SS     SS.  
S%S     S%S  
S%S     S%S  
S%S     S%S  
S&S     S&S  
S&S     S&S  
S&S     S&S  
S*S     S*S  
S*S  .  S*S  
S*S_sSs_S*S  
SSS~SSS~S*S  
             
             
             "#,
r#"
 .S S.   
.SS SS.  
S%S S%S  
S%S S%S  
S%S S%S  
 SS SS   
  S_S    
 SS~SS   
S*S S*S  
S*S S*S  
S*S S*S  
S*S S*S  
SP       
Y        
         "#,
r#"
 .S S.   
.SS SS.  
S%S S%S  
S%S S%S  
S%S S%S  
 SS SS   
  S S    
  SSS    
  S*S    
  S*S    
  S*S    
  S*S    
  SP     
  Y      
         "#,
r#"
 sdSSSSSSSbs  
 YSSSSSSSS%S  
        S%S   
       S&S    
      S&S     
      S&S     
     S&S      
    S*S       
   S*S        
 .s*S         
 sY*SSSSSSSP  
sY*SSSSSSSSP  
              
              
              "#,
"",
"",
"",
"",
"",
"",
r#"
 .S_SSSs    
.SS~SSSSS   
S%S   SSSS  
S%S    S%S  
S%S SSSS%S  
S&S  SSS%S  
S&S    S&S  
S&S    S&S  
S*S    S&S  
S*S    S*S  
S*S    S*S  
SSS    S*S  
       SP   
       Y    
            "#,
r#"
 .S_SSSs    
.SS~SSSSS   
S%S   SSSS  
S%S    S%S  
S%S SSSS%P  
S&S  SSSY   
S&S    S&S  
S&S    S&S  
S*S    S&S  
S*S    S*S  
S*S SSSSP   
S*S  SSY    
SP          
Y           
            "#,
r#"
  sSSs  
 d%%SP  
d%S'    
S%S     
S&S     
S&S     
S&S     
S&S     
S*b     
S*S.    
 SSSbs  
  YSSP  
        
        
        "#,
r#"
 .S_sSSs    
.SS~YS%%b   
S%S   `S%b  
S%S    S%S  
S%S    S&S  
S&S    S&S  
S&S    S&S  
S&S    S&S  
S*S    d*S  
S*S   .S*S  
S*S_sdSSS   
SSS~YSSY    
            
            
            "#,
r#"
  sSSs  
 d%%SP  
d%S'    
S%S     
S&S     
S&S_Ss  
S&S~SP  
S&S     
S*b     
S*S.    
 SSSbs  
  YSSP  
        
        
        "#,
r#"
  sSSs  
 d%%SP  
d%S'    
S%S     
S&S     
S&S_Ss  
S&S~SP  
S&S     
S*b     
S*S     
S*S     
S*S     
SP      
Y       
        "#,
r#"
  sSSSSs  
 d%%%%SP  
d%S'      
S%S       
S&S       
S&S       
S&S       
S&S sSSs  
S*b `S%%  
S*S   S%  
 SS_sSSS  
  Y~YSSY  
          
          
          "#,
r#"
 .S    S.   
.SS    SS.  
S%S    S%S  
S%S    S%S  
S%S SSSS%S  
S&S  SSS&S  
S&S    S&S  
S&S    S&S  
S*S    S*S  
S*S    S*S  
S*S    S*S  
SSS    S*S  
       SP   
       Y    
            "#,
r#"
 .S  
.SS  
S%S  
S%S  
S&S  
S&S  
S&S  
S&S  
S*S  
S*S  
S*S  
S*S  
SP   
Y    
     "#,
r#"
    .S  
   .SS  
   S%S  
   S%S  
   S&S  
   S&S  
   S&S  
   S&S  
   d*S  
  .S*S  
sdSSS   
YSSY    
        
        
        "#,
r#"
 .S    S.   
.SS    SS.  
S%S    S&S  
S%S    d*S  
S&S   .S*S  
S&S_sdSSS   
S&S~YSSY%b  
S&S    `S%  
S*S     S%  
S*S     S&  
S*S     S&  
S*S     SS  
SP          
Y           
            "#,
r#"
S.      
SS.     
S%S     
S%S     
S&S     
S&S     
S&S     
S&S     
S*b     
S*S.    
 SSSbs  
  YSSP  
        
        
        "#,
r#"
 .S_SsS_S.   
.SS~S*S~SS.  
S%S `Y' S%S  
S%S     S%S  
S%S     S%S  
S&S     S&S  
S&S     S&S  
S&S     S&S  
S*S     S*S  
S*S     S*S  
S*S     S*S  
SSS     S*S  
        SP   
        Y    
             "#,
r#"
 .S_sSSs    
.SS~YS%%b   
S%S   `S%b  
S%S    S%S  
S%S    S&S  
S&S    S&S  
S&S    S&S  
S&S    S&S  
S*S    S*S  
S*S    S*S  
S*S    S*S  
S*S    SSS  
SP          
Y           
            "#,
r#"
  sSSs_sSSs    
 d%%SP~YS%%b   
d%S'     `S%b  
S%S       S%S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S*b       d*S  
S*S.     .S*S  
 SSSbs_sdSSS   
  YSSP~YSSY    
               
               
               "#,
r#"
 .S_sSSs    
.SS~YS%%b   
S%S   `S%b  
S%S    S%S  
S%S    d*S  
S&S   .S*S  
S&S_sdSSS   
S&S~YSSY    
S*S         
S*S         
S*S         
S*S         
SP          
Y           
            "#,
r#"
  sSSs_sSSs    
 d%%SP~YS%%b   
d%S'     `S%b  
S%S       S%S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S*b       d*S  
S*S.     .S*S  
 SSSbs_sdSSSS  
  YSSP~YSSSSS  
               
               
               "#,
r#"
 .S_sSSs    
.SS~YS%%b   
S%S   `S%b  
S%S    S%S  
S%S    d*S  
S&S   .S*S  
S&S_sdSSS   
S&S~YSY%b   
S*S   `S%b  
S*S    S%S  
S*S    S&S  
S*S    SSS  
SP          
Y           
            "#,
r#"
  sSSs  
 d%%SP  
d%S'    
S%|     
S&S     
Y&Ss    
`S&&S   
  `S*S  
   l*S  
  .S*P  
sSS*S   
YSS'    
        
        
        "#,
r#"
sdSS_SSSSSSbs  
YSSS~S%SSSSSP  
     S%S       
     S%S       
     S&S       
     S&S       
     S&S       
     S&S       
     S*S       
     S*S       
     S*S       
     S*S       
     SP        
     Y         
               "#,
r#"
 .S       S.   
.SS       SS.  
S%S       S%S  
S%S       S%S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S&S       S&S  
S*b       d*S  
S*S.     .S*S  
 SSSbs_sdSSS   
  YSSP~YSSY    
               
               
               "#,
r#"
 .S    S.   
.SS    SS.  
S%S    S%S  
S%S    S%S  
S&S    S%S  
S&S    S&S  
S&S    S&S  
S&S    S&S  
S*b    S*S  
S*S.   S*S  
 SSSbs_S*S  
  YSSP~SSS  
            
            
            "#,
r#"
 .S     S.   
.SS     SS.  
S%S     S%S  
S%S     S%S  
S%S     S%S  
S&S     S&S  
S&S     S&S  
S&S     S&S  
S*S     S*S  
S*S  .  S*S  
S*S_sSs_S*S  
SSS~SSS~S*S  
             
             
             "#,
r#"
 .S S.   
.SS SS.  
S%S S%S  
S%S S%S  
S%S S%S  
 SS SS   
  S_S    
 SS~SS   
S*S S*S  
S*S S*S  
S*S S*S  
S*S S*S  
SP       
Y        
         "#,
r#"
 .S S.   
.SS SS.  
S%S S%S  
S%S S%S  
S%S S%S  
 SS SS   
  S S    
  SSS    
  S*S    
  S*S    
  S*S    
  S*S    
  SP     
  Y      
         "#,
r#"
 sdSSSSSSSbs  
 YSSSSSSSS%S  
        S%S   
       S&S    
      S&S     
      S&S     
     S&S      
    S*S       
   S*S        
 .s*S         
 sY*SSSSSSSP  
sY*SSSSSSSSP  
              
              
              "#,
"",
"",
"",
"",
];    
