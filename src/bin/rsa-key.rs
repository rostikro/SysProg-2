use std::thread;
use rsa::RsaPrivateKey;

fn rsa_key_1024() -> RsaPrivateKey {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 1024).unwrap();

    private_key
}

fn rsa_key_2048() -> RsaPrivateKey {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();

    private_key
}

fn rsa_key_4096() -> RsaPrivateKey {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 4096).unwrap();

    private_key
}

fn main () {
    let mut threads = vec![];

    {
        let h = thread::spawn(|| rsa_key_1024());
        threads.push(h);
    }

    {
        let h = thread::spawn(|| rsa_key_2048());
        threads.push(h);
    }

    {
        let h = thread::spawn(|| rsa_key_4096());
        threads.push(h);
    }


    for handle in threads {
        handle.join().unwrap();
    }

    println!("Success");
}