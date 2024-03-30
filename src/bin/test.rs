use algo_nt;
use simple_logger;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
//    let n = algo_nt::pollard_rho_factorisation(65, None);
    //println!("{}", n);
    println!("32 is {}", algo_nt::miller_rabin_test(32, 25));
    println!("1109 is {}", algo_nt::miller_rabin_test(1109, 25));
    println!("17 is {}", algo_nt::miller_rabin_test(17, 25));
    println!("13892533 is {}", algo_nt::miller_rabin_test(13892533, 25));
    println!("13892527 is {}", algo_nt::miller_rabin_test(13892527, 25));
    println!("factors for {} are {:?}", 65, algo_nt::pollard_rho_factorisation(65, None));
    
    println!("factors for {} are {:?}", 13892527, algo_nt::pollard_rho_factorisation(13892527, None));
}
