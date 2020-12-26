use flimflam_model::Event;
use std::io::BufReader;
use std::net::TcpListener;

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;

    for stream in listener.incoming() {
        let mut stream = BufReader::new(stream?);

        loop {
            let event: Event = jsonl::read(&mut stream)?;
            dbg!(event);
        }
    }

    Ok(())
}
