    use crate::Message; 
    use anyhow::bail;
    use anyhow::Result;
    use core::fmt::Debug;

    use uuid::Uuid;

    //Using this trait as an interface to abstract the actual transmission
    //mechanism
    pub trait HandshakeInterface
    {
        fn initialize(&mut self) -> Result<()> {
            Ok(())  //default implementation
        }
        fn send_message(&mut self, message: Message) -> Result<()>;
    }

    
    /*impl Debug for dyn HandshakeInterface {
        fn fmt(&'a self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "HandshakeInterface")
        }
    }*/

    //Store Handshake state
    //#[derive(Debug)]
    pub struct HandshakeProcess<'a>
    {
        state: HandshakeState,
        role: HandshakeRole,
        interface: &'a mut dyn HandshakeInterface,
        uuid: Uuid,

    }

    #[derive(Debug)]
    pub enum HandshakeRole {
        Upstairs,
        Downstairs,
    }

    #[derive(Debug)]
    pub enum HandshakeState {
        Start,
        WaitForActive,
        RegionInfo,
        ExtentVersion,
        Complete,
    }

    impl<'a> HandshakeProcess<'a>
    {
        pub fn new(role: HandshakeRole, interface: &mut dyn HandshakeInterface, uuid: Uuid) -> HandshakeProcess 
        {
            HandshakeProcess {
                state: HandshakeState::Start,
                role,
                interface,
                uuid,
            }
        }

        pub fn start(&mut self) -> Result<()> {           
            if let Err(error) = self.interface.initialize() {
                return Err(error);
            }
            match self.role {
                HandshakeRole::Upstairs => { self.interface.send_message(Message::HereIAm(1, self.uuid)) }
                HandshakeRole::Downstairs => { Ok(()) }  //TBD
            }
        }

        pub fn process_message(&mut self, message: Message) -> Result<()> {
            match message {
                Message::Imok => { return Ok(()) } //noop in all states
                message => {
                    match self.state.process_message(message) {
                        Ok(new_state) => {
                            self.state = new_state;
                            Ok(())
                        }
                        Err(error_message) => { bail!(error_message) }
                    }
                }
            }
        }

    }

    impl HandshakeState {
        fn process_message(&self, message: Message) -> Result<HandshakeState> {
            match (self, message) {
                (HandshakeState::Start, Message::YesItsMe(version)) => { 
                    return Ok(HandshakeState::WaitForActive)
                }
                (_s, Message::YesItsMe(version)) => { 
                    bail!("Got version already!");
                }
                (s, m) => {
                    bail!(
                     "Unexpected command {:?} received in state {:#?}",
                         m, s);
                }
            };
        }
        
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        
        struct HandshakeTestInterface {
            last_message :Option<Message>,
        }

        impl HandshakeInterface for HandshakeTestInterface  {
            fn initialize(&mut self) -> Result<()>{
                println!("HandshakeTestInterface->Initialize");
                Ok(())
            }
            fn send_message(&mut self, message: Message) -> Result<()>
            {
                println!("Message Sent {:#?}!", message);
                self.last_message = Some(message);
                Ok(())
            }
        }
            

        #[test]
        fn init_upstairs_test() {
            let mut test_interface = HandshakeTestInterface { last_message: None };
            let uuid = uuid::Uuid::new_v4();
            let mut handshake = HandshakeProcess::new(HandshakeRole::Upstairs, &mut test_interface, uuid);
            assert!(matches!(handshake.start(), Ok(()) ));            
            assert!(matches!(test_interface.last_message, Some(Message::HereIAm(1, uuid))));
            //assert!(matches!(handshake.process_message(Message::YesItsMe(1)), Ok(()) ));                                    
        }

        #[test]
        fn init_downstairs_test() {
            let mut test_interface = HandshakeTestInterface { last_message: None };
            let uuid = uuid::Uuid::new_v4();
            let mut handshake = HandshakeProcess::new(HandshakeRole::Downstairs, &mut test_interface, uuid);
            assert!(matches!(handshake.start(), Ok(()) ));            
            assert!(matches!(test_interface.last_message, None));            
        }

        #[test]
        fn imok_on_start_test() {

            let mut test_interface = HandshakeTestInterface { last_message: None };
            let mut handshake = HandshakeProcess::new(HandshakeRole::Upstairs, &mut test_interface, uuid::Uuid::new_v4());
            assert!(matches!(handshake.start(), Ok(()) ));            
            assert!(matches!(handshake.process_message(Message::Imok), Ok(()) ));
            assert!(matches!(test_interface.last_message, Some(Message::HereIAm(1, uuid))));

        }
/*
        #[test]
        fn repeat_yesitsme_should_fail_test() {
            let mut Handshake = HandshakeProcess::new(true, SendTestCallback, uuid::Uuid::new_v4());
            if let Ok(()) = Handshake.process_message(Message::YesItsMe(1)) {
                if let Ok(()) = Handshake.process_message(Message::YesItsMe(1)) {
                     assert!(false);
                }
            } else {
                assert!(false);
            }
        }

        #[test]
        fn out_of_sequence_message_in_start() {
            let uuid = uuid::Uuid::new_v4();
            let mut Handshake = HandshakeProcess::new(true, SendTestCallback, uuid);
            if let Ok(()) = Handshake.process_message(Message::YouAreNowActive(uuid)) {
                assert!(false);
            }
        }
        */
    }
