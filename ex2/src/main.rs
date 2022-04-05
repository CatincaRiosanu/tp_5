use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, write, getpid, getppid}};

fn main() {
    // match unsafe{fork()} { 
    //     Ok(ForkResult::Parent{ child, .. }) => {
    //         println!("Child pid: {}", child);
    //         waitpid(child, None).unwrap();
    //     }

    //     Ok(ForkResult::Child) => {
    //         // println!("Child pid: {}", child);
    //         // waitpid(child, None).unwrap();
    //     }

    //     Err(_) => println!("Fork failed"),
       
    // }

    match unsafe{fork()} { 
         Ok(result) => {
             match result {
                 ForkResult::Parent { child } => {
                    println!("Eu sunt parintele si pid-ul meu este: {}", getpid());
                    println!("Copilul meu este (pid): {}", child);
                 }
                 ForkResult::Child => {
                     println!("Eu sunt copilul si pid-ul meu este: {}", getpid());
                    println!("Parintele meu este (pid): {}", getppid())

                 }
             }
         }

         Err(_) => println!("Fork failed"),
       
     }
}

