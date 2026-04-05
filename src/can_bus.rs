use embedded_can::{Id, StandardId};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]

pub struct DME1 {
    pub bitfield: u8, // Byte 0 raw

    pub indexed_torque_x100: u16, // % * 100
    pub engine_speed_rpm: u16,
    pub indicated_torque_x100: u16,
    pub torque_loss_x100: u16,
    pub vehicle_speed_kmh: u8,
    pub theoretical_torque_x100: u16,
}

impl DME1 {
    pub fn parse(data: &[u8; 8]) -> Self {
        u16::from_le_bytes([data[3], data[2]]);
        let rpm_raw = ((data[3] as u16) << 8) | data[2] as u16;

        Self {
            bitfield: data[0],

            indexed_torque_x100: (data[1] as u16) * 39,
            engine_speed_rpm: rpm_raw * 5 / 32, // 0.15625 = 5/32
            indicated_torque_x100: (data[4] as u16) * 39,
            torque_loss_x100: (data[5] as u16) * 39,
            vehicle_speed_kmh: data[6],
            theoretical_torque_x100: (data[7] as u16) * 39,
        }
    }

    // Bit 0 - CAN Version (CAN_VERS_CAN),
    // Bit 1 - TCU configuration (CONF_TCU_CAN)
    // Bits 2-3 - OBD freeze frame (OBD_FRF_ACK_CAN) @TODO: Not sure about the length
    // Bit 4-5 - Torque scaling factor (TQ_STND_CAN) (6 bits long) @TODO: Not sure about the length
    // Bit 6 - MUL_CODE: Identification of MUL_INFO
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct DME2 {
    pub flags: u8,

    pub coolant_temp_c_x100: i16,
    pub ambient_pressure_hpa: u16,
    pub ambient_pressure_valid: u8,

    pub battery_connected: u8,
    pub ack_engine_stopped: u8,

    pub pedal_position_x100: u16,
    pub pedal_valid: u8,

    pub engine_characteristic: u8,
}

impl DME2 {
    pub fn parse(data: &[u8; 8]) -> Self {
        let coolant = (data[1] as i16 * 75) - 4800;

        let (pressure, pressure_valid) = if data[2] == 0xFF {
            (0, 0)
        } else {
            (((data[2] as u16) * 2) + 598, 1)
        };

        let (pedal, pedal_valid) = if data[5] == 0xFF {
            (0, 0)
        } else {
            (((data[5] as u16) * 25) / 64, 1)
        };

        Self {
            flags: data[0],
            coolant_temp_c_x100: coolant,
            ambient_pressure_hpa: pressure,
            ambient_pressure_valid: pressure_valid,
            battery_connected: (data[3] == 0x10) as u8,
            ack_engine_stopped: data[4],
            pedal_position_x100: pedal,
            pedal_valid,
            engine_characteristic: data[6] & 0x1F,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct DME4 {
    pub flags: u8,

    pub fuel_consumption_x128: u32, // raw * 128
    pub battery_voltage_dv: u16,    // decivolts (0.1V)
}

impl DME4 {
    pub fn parse(data: &[u8; 8]) -> Self {
        let fco_raw = ((data[2] as u16) << 8) | data[1] as u16;

        Self {
            flags: data[0],
            fuel_consumption_x128: (fco_raw as u32) * 128,
            battery_voltage_dv: (data[3] as u16),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum CanBusParseError {
    InvalidFrameType,
    InvalidDataLength,
    UnknownMessageType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum CanBusMessage {
    DME1(DME1),
    DME2(DME2),
    DME4(DME4),
}

pub const DME1_CAN_ID: StandardId = StandardId::new(0x130).unwrap();
pub const DME2_CAN_ID: StandardId = StandardId::new(0x132).unwrap();
pub const DME4_CAN_ID: StandardId = StandardId::new(0x134).unwrap();

impl CanBusMessage {
    pub fn try_from_embedded_can_frame(
        frame: impl embedded_can::Frame,
    ) -> Result<Self, CanBusParseError> {
        // All message is standard frame
        let Id::Standard(id) = frame.id() else {
            return Err(CanBusParseError::InvalidFrameType);
        };
        // All message has 8 bytes of data
        let data = frame.data();
        if data.len() != 8 {
            return Err(CanBusParseError::InvalidDataLength);
        }
        let data: &[u8; 8] = frame.data().try_into().unwrap();

        match id {
            DME1_CAN_ID => Ok(Self::DME1(DME1::parse(data))),
            DME2_CAN_ID => Ok(Self::DME2(DME2::parse(data))),
            DME4_CAN_ID => Ok(Self::DME4(DME4::parse(data))),
            _ => Err(CanBusParseError::UnknownMessageType),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         // assert_eq!(4, add_two(2));
//         let frame = embedded_can::Frame::new(
//             DME1_CAN_ID,
//             &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
//         )
//         .unwrap();
//         CanBusMessage::try_from_embedded_can_frame(frame).unwrap();
//     }
// }

#[cfg(test)]
mod tests {
    use embedded_can::Frame;

    use super::*;

    #[test]
    fn it_works() {
        let frame = can::frame::Frame::from_static(
            can::identifier::Id::Standard(DME1_CAN_ID.into()),
            &[0x05, 0xB9, 0xFC, 0x30, 0xB9, 0x06, 0x68, 0x7C],
        );

        let frame = CanBusMessage::try_from_embedded_can_frame(frame).unwrap();

        assert_eq!(
            frame,
            CanBusMessage::DME1(DME1 {
                bitfield: 0x05,
                indexed_torque_x100: 0xB9 * 39,
                engine_speed_rpm: ((0x30 as u16) << 8 | 0xFC as u16) * 5 / 32,
                indicated_torque_x100: 0xB9 * 39,
                torque_loss_x100: 234,
                vehicle_speed_kmh: 62,
                theoretical_torque_x100: 0,
            })
        );
    }
}
