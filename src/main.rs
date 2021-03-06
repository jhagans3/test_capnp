extern crate capnp;

pub mod addressbook_capnp {
  include!(concat!(env!("OUT_DIR"), "/src/schemas/addressbook_capnp.rs"));
  //include!("schemas/addressbook_capnp.rs");
}

pub mod addressbook {
    use addressbook_capnp::{address_book, person};
    use capnp::serialize_packed;

    pub fn write_address_book() -> ::std::io::Result<()> {
        let mut message = ::capnp::message::Builder::new_default();
        {
            let address_book = message.init_root::<address_book::Builder>();

            let mut people = address_book.init_people(2);

            {
                let mut alice = people.borrow().get(0);
                alice.set_id(123);
                alice.set_name("Alice");
                alice.set_email("alice@example.com");
                {
                    let mut alice_phones = alice.borrow().init_phones(1);
                    alice_phones.borrow().get(0).set_number("555-1212");
                    alice_phones.borrow().get(0).set_type(person::phone_number::Type::Mobile);
                }
                alice.get_employment().set_school("MIT");
            }

            {
                let mut bob = people.get(1);
                bob.set_id(456);
                bob.set_name("Bob");
                bob.set_email("bob@example.com");
                {
                    let mut bob_phones = bob.borrow().init_phones(2);
                    bob_phones.borrow().get(0).set_number("555-4567");
                    bob_phones.borrow().get(0).set_type(person::phone_number::Type::Home);
                    bob_phones.borrow().get(1).set_number("555-7654");
                    bob_phones.borrow().get(1).set_type(person::phone_number::Type::Work);
                }
                bob.get_employment().set_unemployed(());
            }
        }

        serialize_packed::write_message(&mut ::std::io::stdout(), &message)
    }

    pub fn print_address_book() -> ::capnp::Result<()> {

        let stdin = ::std::io::stdin();
        let message_reader = try!(serialize_packed::read_message(&mut stdin.lock(),
                                                                 ::capnp::message::ReaderOptions::new()));
        let address_book = try!(message_reader.get_root::<address_book::Reader>());

        for person in try!(address_book.get_people()).iter() {
            println!("{}: {}", try!(person.get_name()), try!(person.get_email()));
            for phone in try!(person.get_phones()).iter() {
                let type_name = match phone.get_type() {
                    Ok(person::phone_number::Type::Mobile) => "mobile",
                    Ok(person::phone_number::Type::Home) => "home",
                    Ok(person::phone_number::Type::Work) => "work",
                    Err(::capnp::NotInSchema(_)) => "UNKNOWN",
                };
                println!("  {} phone: {}", type_name, try!(phone.get_number()));
            }
            match person.get_employment().which() {
                Ok(person::employment::Unemployed(())) => {
                    println!("  unemployed");
                }
                Ok(person::employment::Employer(employer)) => {
                    println!("  employer: {}", try!(employer));
                }
                Ok(person::employment::School(school)) => {
                    println!("  student at: {}", try!(school));
                }
                Ok(person::employment::SelfEmployed(())) => {
                    println!("  self-employed");
                }
                Err(::capnp::NotInSchema(_)) => { }
            }
        }
        Ok(())
    }
}

pub fn main() {

    let args : Vec<String> = ::std::env::args().collect();
    if args.len() < 2 {
        println!("usage: $ {} [write | read]", args[0]);
    } else {
        match &*args[1] {
            "write" => addressbook::write_address_book().unwrap(),
            "read" =>  addressbook::print_address_book().unwrap(),
            _ => {println!("unrecognized argument") }
        }
    }

}
