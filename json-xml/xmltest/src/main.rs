extern crate quick_xml;

fn main() {
    //println!("Hello, world!");
    use quick_xml::reader::Reader;
    use quick_xml::events::Event;

    let xml = "<tag1>text1</tag1><tag1>text2</tag1><tag1>text3</tag1><tag1><tag2>text4世界</tag2></tag1>";

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"tag2" => {
                txt.push(
                    reader.read_text(b"tag2", &mut Vec::new()).expect("Cannot decode text value"),
                    );
                println!("{:?}", txt);
            }
            Ok(Event::Eof) => break, //exit loop if end of file is reached.
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other event's which we do not consider here.
        }
        buf.clear();
    }
}
