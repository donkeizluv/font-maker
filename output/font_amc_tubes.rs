
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
d s.   
S  ~O  
S   `b 
S sSSO 
S    O 
S    O 
P    P 
       "#,
r#"
d ss.  
S    b 
S    P 
S sSS' 
S    b 
S    P 
P `SS  
       "#,
r#"
  sSSs. 
 S      
S       
S       
S       
 S      
  "sss' 
        "#,
r#"
d ss    
S   ~o  
S     b 
S     S 
S     P 
S    S  
P ss"   
        "#,
r#"
d sss   
S       
S       
S sSSs  
S       
S       
P sSSss 
        "#,
r#"
d sss  
S      
S      
S sSSs 
S      
S      
P      
       "#,
r#"
  sSSSs   
 S     S  
S         
S         
S    ssSb 
 S     S  
  "sss"   
          "#,
r#"
d    d 
S    S 
S    S 
S sSSS 
S    S 
S    S 
P    P 
       "#,
r#"
d 
S 
S 
S 
S 
S 
P 
  "#,
r#"
        d 
        S 
        S 
        S 
d       P 
 S     S  
  "sss"   
          "#,
r#"
d     S 
S    P  
Ssss'   
S   s   
S    b  
S     b 
P     P 
        "#,
r#"
d      
S      
S      
S      
S      
S      
P sSSs 
       "#,
r#"
d s   sb 
S  S S S 
S   S  S 
S      S 
S      S 
S      S 
P      P 
         "#,
r#"
d s  b 
S  S S 
S   SS 
S    S 
S    S 
S    S 
P    P 
       "#,
r#"
  sSSSs   
 S     S  
S       S 
S       S 
S       S 
 S     S  
  "sss"   
          "#,
r#"
d ss.  
S    b 
S    P 
S sS'  
S      
S      
P      
       "#,
r#"
  sSSSs   
 S     S  
S       S 
S       S 
S       S 
 S   s S  
  "sss"ss 
          "#,
r#"
d ss.  
S    b 
S    P 
S sS'  
S   S  
S    S 
P    P 
       "#,
r#"
  sss. 
d      
Y      
  ss.  
     b 
     P 
` ss'  
       "#,
r#"
sss sssss 
    S     
    S     
    S     
    S     
    S     
    P     
          "#,
r#"
d       b 
S       S 
S       S 
S       S 
S       S 
 S     S  
  "sss"   
          "#,
r#"
d    b 
S    S 
S    S 
S    S 
S    S 
 S   S 
  "ssS 
       "#,
r#"
d  d  b 
S  S  S 
S  S  S 
S  S  S 
S  S  S 
 S  S S 
  "ss"S 
        "#,
r#"
Ss   sS 
  S S   
   S    
   S    
   S    
  S S   
s"   "s 
        "#,
r#"
Ss   sS 
  S S   
   S    
   S    
   S    
   S    
   P    
        "#,
r#"
sSSSSSs 
     s  
    s   
   s    
  s     
 s      
sSSSSSs 
        "#,
"",
"",
"",
"",
"",
"",
r#"
d s.   
S  ~O  
S   `b 
S sSSO 
S    O 
S    O 
P    P 
       "#,
r#"
d ss.  
S    b 
S    P 
S sSS' 
S    b 
S    P 
P `SS  
       "#,
r#"
  sSSs. 
 S      
S       
S       
S       
 S      
  "sss' 
        "#,
r#"
d ss    
S   ~o  
S     b 
S     S 
S     P 
S    S  
P ss"   
        "#,
r#"
d sss   
S       
S       
S sSSs  
S       
S       
P sSSss 
        "#,
r#"
d sss  
S      
S      
S sSSs 
S      
S      
P      
       "#,
r#"
  sSSSs   
 S     S  
S         
S         
S    ssSb 
 S     S  
  "sss"   
          "#,
r#"
d    d 
S    S 
S    S 
S sSSS 
S    S 
S    S 
P    P 
       "#,
r#"
d 
S 
S 
S 
S 
S 
P 
  "#,
r#"
        d 
        S 
        S 
        S 
d       P 
 S     S  
  "sss"   
          "#,
r#"
d     S 
S    P  
Ssss'   
S   s   
S    b  
S     b 
P     P 
        "#,
r#"
d      
S      
S      
S      
S      
S      
P sSSs 
       "#,
r#"
d s   sb 
S  S S S 
S   S  S 
S      S 
S      S 
S      S 
P      P 
         "#,
r#"
d s  b 
S  S S 
S   SS 
S    S 
S    S 
S    S 
P    P 
       "#,
r#"
  sSSSs   
 S     S  
S       S 
S       S 
S       S 
 S     S  
  "sss"   
          "#,
r#"
d ss.  
S    b 
S    P 
S sS'  
S      
S      
P      
       "#,
r#"
  sSSSs   
 S     S  
S       S 
S       S 
S       S 
 S   s S  
  "sss"ss 
          "#,
r#"
d ss.  
S    b 
S    P 
S sS'  
S   S  
S    S 
P    P 
       "#,
r#"
  sss. 
d      
Y      
  ss.  
     b 
     P 
` ss'  
       "#,
r#"
sss sssss 
    S     
    S     
    S     
    S     
    S     
    P     
          "#,
r#"
d       b 
S       S 
S       S 
S       S 
S       S 
 S     S  
  "sss"   
          "#,
r#"
d    b 
S    S 
S    S 
S    S 
S    S 
 S   S 
  "ssS 
       "#,
r#"
d  d  b 
S  S  S 
S  S  S 
S  S  S 
S  S  S 
 S  S S 
  "ss"S 
        "#,
r#"
Ss   sS 
  S S   
   S    
   S    
   S    
  S S   
s"   "s 
        "#,
r#"
Ss   sS 
  S S   
   S    
   S    
   S    
   S    
   P    
        "#,
r#"
sSSSSSs 
     s  
    s   
   s    
  s     
 s      
sSSSSSs 
        "#,
"",
"",
"",
"",
];    
