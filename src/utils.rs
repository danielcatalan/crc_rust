#[macro_export]
macro_rules! c_for {
    ($a:stmt; $b:expr; $c:expr; $d:block) => {
        {
            $a
            while $b{
                $d
                $c;
            }
        }  
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_c_for() {
        let mut x = 1;
    
        // factorial of 5
        // !5 => 5*4*3*2*1 => 120
        c_for!(let mut i=1; i <= 5; i+=1; {
            x *= i;
        });
    
        assert_eq!(120, x);
    }
}

