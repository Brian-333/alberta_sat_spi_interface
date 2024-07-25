/// Ports used for communication between handlers and simulated subsystems / payloads
pub mod ports {
    pub const SIM_DFGM_PORT: u16 = 1802;
    pub const SIM_ADCS_PORT: u16 = 1803;
    pub const SIM_EPS_PORT: u16 = 1804;
    pub const SIM_COMMS_PORT: u16 = 1805;
    pub const SIM_IRIS_PORT: u16 = 1806;

    pub const DFGM_HANDLER_DISPATCHER_PORT: u16 = 1900;
    pub const SCHEDULER_DISPATCHER_PORT: u16 = 1901;
    pub const SUBSYSTEM_MONITOR_DISPATCHER_PORT: u16 = 1902;
    pub const BULK_MSG_HANDLER_DISPATCHER_PORT: u16 = 1903;
}

/// Each thing that can emit or receive a message has an associated ID. Each message header includes this id for source and destination.
/// Referencing this page:
pub mod component_ids {
    use std::fmt;
    use std::str::FromStr;

    pub const OBC: u8 = 0;
    pub const EPS: u8 = 1;
    pub const ADCS: u8 = 2;
    pub const DFGM: u8 = 3;
    pub const IRIS: u8 = 4;
    pub const GPS: u8 = 5;
    //.....
    pub const GS: u8 = 7;
    pub const COMS: u8 = 8;

    #[derive(PartialEq, Debug)]
    pub enum ComponentIds {
        OBC = 0,
        EPS = 1,
        ADCS = 2,
        DFGM = 3,
        IRIS = 4,
        GPS = 5,
        //...
        GS = 7,
        COMS = 8,
        //..
        //..
        TEST = 99,
    }

    impl fmt::Display for ComponentIds {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                ComponentIds::OBC => write!(f, "OBC"),
                ComponentIds::EPS => write!(f, "EPS"),
                ComponentIds::ADCS => write!(f, "ADCS"),
                ComponentIds::DFGM => write!(f, "DFGM"),
                ComponentIds::IRIS => write!(f, "IRIS"),
                ComponentIds::GPS => write!(f, "GPS"),
                ComponentIds::GS => write!(f, "GS"),
                ComponentIds::COMS => write!(f, "COMS"),
                ComponentIds::TEST => write!(f, "TEST"),
            }
        }
    }
    impl FromStr for ComponentIds {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "OBC" => Ok(ComponentIds::OBC),
                "EPS" => Ok(ComponentIds::EPS),
                "ADCS" => Ok(ComponentIds::ADCS),
                "DFGM" => Ok(ComponentIds::DFGM),
                "IRIS" => Ok(ComponentIds::IRIS),
                "GPS" => Ok(ComponentIds::GPS),
                "GS" => Ok(ComponentIds::GS),
                "COMS" => Ok(ComponentIds::COMS),
                "TEST" => Ok(ComponentIds::TEST),
                _ => Err(()),
            }
        }
    }

    impl From<u8> for ComponentIds {
        fn from(value: u8) -> Self {
            match value {
                0 => ComponentIds::OBC,
                1 => ComponentIds::EPS,
                2 => ComponentIds::ADCS,
                3 => ComponentIds::DFGM,
                4 => ComponentIds::IRIS,
                5 => ComponentIds::GPS,
                7 => ComponentIds::GS,
                8 => ComponentIds::COMS,
                99 => ComponentIds::TEST,
                _ => {
                    eprintln!("Invalid component id: {}", value);
                    ComponentIds::TEST // or choose a default value or handle the error in a different way
                }
            }
        }
    }
}

/// For constants that are used across the entire project
pub mod constants {
    pub const UHF_MAX_MESSAGE_SIZE_BYTES: u8 = 128;
}

/// Here opcodes and their associated meaning are defined for each component
/// This is in common lib because components will need to know what opcodes to use when sending messages to other components
/// For example if a message is sent to the OBC to get housekeeping data,
pub mod opcodes {
    pub mod coms {
        pub const GET_HK: u8 = 3;
        pub const SET_BEACON: u8 = 4;
        pub const GET_BEACON: u8 = 5;
    }
    pub mod dfgm {
        pub const TOGGLE_DATA_COLLECTION: u8 = 0;
        pub const GET_DFGM_DATA: u8 = 1;
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::component_ids;

    #[test]
    fn get_component_enum_from_u8() {
        //Test conversion from u8 to ComponentIds enum for all
        let eps = component_ids::ComponentIds::from(1);
        assert_eq!(eps, component_ids::ComponentIds::EPS);

        let adcs = component_ids::ComponentIds::from(2);
        assert_eq!(adcs, component_ids::ComponentIds::ADCS);

        let dfgm = component_ids::ComponentIds::from(3);
        assert_eq!(dfgm, component_ids::ComponentIds::DFGM);

        let iris = component_ids::ComponentIds::from(4);
        assert_eq!(iris, component_ids::ComponentIds::IRIS);

        let gps = component_ids::ComponentIds::from(5);
        assert_eq!(gps, component_ids::ComponentIds::GPS);

        let gs = component_ids::ComponentIds::from(7);
        assert_eq!(gs, component_ids::ComponentIds::GS);

        let coms = component_ids::ComponentIds::from(8);
        assert_eq!(coms, component_ids::ComponentIds::COMS);

        let test = component_ids::ComponentIds::from(99);
        assert_eq!(test, component_ids::ComponentIds::TEST);

        let obc = component_ids::ComponentIds::from(0);
        assert_eq!(obc, component_ids::ComponentIds::OBC);
    }

    #[test]
    fn get_component_enum_from_str() {
        let eps = component_ids::ComponentIds::from_str("EPS").unwrap();
        assert_eq!(eps, component_ids::ComponentIds::EPS);

        let adcs = component_ids::ComponentIds::from_str("ADCS").unwrap();
        assert_eq!(adcs, component_ids::ComponentIds::ADCS);

        let dfgm = component_ids::ComponentIds::from_str("DFGM").unwrap();
        assert_eq!(dfgm, component_ids::ComponentIds::DFGM);

        let iris = component_ids::ComponentIds::from_str("IRIS").unwrap();
        assert_eq!(iris, component_ids::ComponentIds::IRIS);

        let gps = component_ids::ComponentIds::from_str("GPS").unwrap();
        assert_eq!(gps, component_ids::ComponentIds::GPS);

        let gs = component_ids::ComponentIds::from_str("GS").unwrap();
        assert_eq!(gs, component_ids::ComponentIds::GS);

        let coms = component_ids::ComponentIds::from_str("COMS").unwrap();
        assert_eq!(coms, component_ids::ComponentIds::COMS);

        let test = component_ids::ComponentIds::from_str("TEST").unwrap();
        assert_eq!(test, component_ids::ComponentIds::TEST);

        let obc = component_ids::ComponentIds::from_str("OBC").unwrap();
        assert_eq!(obc, component_ids::ComponentIds::OBC);
    }

    #[test]
    fn get_component_str_from_enum() {
        let eps = component_ids::ComponentIds::EPS;
        assert_eq!(eps.to_string(), "EPS");

        let adcs = component_ids::ComponentIds::ADCS;
        assert_eq!(adcs.to_string(), "ADCS");

        let dfgm = component_ids::ComponentIds::DFGM;
        assert_eq!(dfgm.to_string(), "DFGM");

        let iris = component_ids::ComponentIds::IRIS;
        assert_eq!(iris.to_string(), "IRIS");

        let gps = component_ids::ComponentIds::GPS;
        assert_eq!(gps.to_string(), "GPS");

        let gs = component_ids::ComponentIds::GS;
        assert_eq!(gs.to_string(), "GS");

        let coms = component_ids::ComponentIds::COMS;
        assert_eq!(coms.to_string(), "COMS");

        let test = component_ids::ComponentIds::TEST;
        assert_eq!(test.to_string(), "TEST");

        let obc = component_ids::ComponentIds::OBC;
        assert_eq!(obc.to_string(), "OBC");
    }
}
