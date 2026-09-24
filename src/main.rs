use std::sync::{Arc, Mutex, RwLock} ;
use std::thread ;

fn main() {
    
    // Использование Arc<RwLock<i32>> с thread
    let v = Arc::new(RwLock::new(10)) ;

    let my_hand = thread::spawn({
        let v_clone = v.clone() ;
        move || {
            match v_clone.write() {
                Ok(mut r) => {
                    *r += 10 ;
                    Ok(*r)
                },
                Err(err) => {
                    Err(format!("{}", err))
                },
            }
        }
    }) ;

    match my_hand.join() {
        Ok(res) => {
            match res {
                Ok(r) => println!("result: {}", r), // Out: result: 20
                Err(err) => eprintln!("Error: {}", err),
            }
        },
        Err(err) => println!("thread error: {:?}", err),
    }

    println!("Readed data: {}", *v.read().unwrap()) ;   // Out: Readed data: 20

    // Использование Mutex
    let m = Arc::new(Mutex::new(30)) ;

    let my_hand_2 = thread::spawn({
        let m_clone = m.clone() ;
        move || {
            match m_clone.lock() {
                Ok(mut res) => {
                    *res += 20 ;
                    Ok(*res)
                },
                Err(err) => Err(format!("{}", err)),
            }
        }
    }) ;

    match my_hand_2.join() {
        Ok(res) => {
            match res {
                Ok(r) => println!("result: {r}"),  // Out: result: 50
                Err(err) => eprintln!("Error: {err}"),
            }
        },
        Err(err) => eprintln!("thread error: {err:?}"),
    }

}
