use embedded_can::{Id, StandardId};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct DME1 {
    data: [u8; 8],
}

impl DME1 {
    pub fn parse(data: [u8; 8]) -> Self {
        Self { data }
    }

    /// Terminal 15 –KEY ON
    pub fn swi_igk_can(&self) -> bool {
        self.data[0] & 0x01 != 0
    }

    /// Error – engine speed signal
    pub fn f_n_eng_can(&self) -> bool {
        self.data[0] & 0x02 != 0
    }

    /// Acknowledgement of TCS. This bit is set to 0 if no CAN messages from the TCS were received for at least 500ms
    pub fn ack_tcs_can(&self) -> bool {
        self.data[0] & 0x04 != 0
    }

    /// Engine in fuel cut off (LV_PUC - Trailing Throttle Fuel Cut Off)
    pub fn puc_stat(&self) -> bool {
        self.data[0] & 0x08 != 0
    }

    /// Status of torque intervention 0 - The desired intervention regarding ignition angle retardation and cylinder shut-off is executed. (Default value) 1 - The desired intervention regarding ignition angle retardation and cylinder shut-off is executed; however, the requested target torque can not be adjusted precisely (torque steps) 2 - The torque reduction regarding the ignition angle retardation cannot be completely executed. A cylinder shut-off is not possible at this time. Therefore a remaining torque (as difference between TQI_ASR/GS_REQ and TQI_AV) is present and cannot be reduced. 3 - The desired torque intervention for TCS regarding the ignition angle and cylinder shut-off can no longer be executed. The torque intervention is terminated, the engine management system resets the requested engine torque to the TQI value using a ramp.
    pub fn tq_cor_stat_can(&self) -> bool {
        self.data[0] & 0x10 != 0
    }

    /// Activation, air conditioner compressor relay
    pub fn rly_ac_can(&self) -> bool {
        self.data[0] & 0x40 != 0
    }

    /// Error on torque measure or calculation
    pub fn f_sub_tqi_can(&self) -> bool {
        self.data[0] & 0x80 != 0
    }

    /// Indexed Engine Torque in % of C_TQ_STND (including ASR/MSR/ETCU/LIM/AMT/GEAR intervention)
    pub fn tqi_tqr_can(&self) -> f32 {
        self.data[1] as f32 * 0.39
    }

    /// Indexed Engine Torque in % (×100) of C_TQ_STND (including ASR/MSR/ETCU/LIM/AMT/GEAR intervention)
    pub fn tqi_tqr_can_x100(&self) -> u16 {
        self.data[1] as u16 * 39
    }

    /// Engine Speed in rpm
    pub fn n_eng(&self) -> f32 {
        u16::from_le_bytes([self.data[2], self.data[3]]) as f32 * 0.15625
    }

    /// Engine Speed in rpm (in integer)
    pub fn n_eng_int(&self) -> u16 {
        u16::from_le_bytes([self.data[2], self.data[3]]) * 5 / 32
    }

    /// Indicated Engine Torque in % of C_TQ_STND (based on PVS, N, AMP, TIA ,TCO, IGA, PUC so ip_tqi_pvs__n__pvs)
    pub fn tqi_can(&self) -> f32 {
        self.data[4] as f32 * 0.39
    }

    /// Indicated Engine Torque in % (x100) of C_TQ_STND (based on PVS, N, AMP, TIA ,TCO, IGA, PUC so ip_tqi_pvs__n__pvs)
    pub fn tqi_can_x100(&self) -> u16 {
        self.data[4] as u16 * 39
    }

    /// Engine Torque Loss (due to engine friction, AC compressor and electrical power consumption) in % of C_TQ_STND
    pub fn tq_loss_can(&self) -> f32 {
        self.data[5] as f32 * 0.39
    }

    /// Engine Torque Loss (due to engine friction, AC compressor and electrical power consumption) in % (x100) of C_TQ_STND
    pub fn tq_loss_can_x100(&self) -> u16 {
        self.data[5] as u16 * 39
    }

    /// Vehicle speed in km/h
    pub fn vs(&self) -> u8 {
        self.data[6]
    }

    /// **Needs verification**
    ///
    /// Theorethical Engine Torque in % of C_TQ_STND after charge intervention (based on MAF & IGA so ip_tqi_maf__n__maf)
    pub fn tqi_maf_can(&self) -> f32 {
        self.data[7] as f32 * 0.39
    }

    /// **Needs verification**
    ///
    /// Theorethical Engine Torque in % of C_TQ_STND after charge intervention (based on MAF & IGA so ip_tqi_maf__n__maf)
    pub fn tqi_maf_can_x100(&self) -> u16 {
        self.data[7] as u16 * 39
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct DME2 {
    data: [u8; 8],
}

impl DME2 {
    pub fn parse(data: [u8; 8]) -> Self {
        Self { data }
    }

    /// Bit 0 - CAN Version
    pub fn can_version(&self) -> bool {
        self.data[0] & 0x01 != 0
    }

    /// Bit 1 - TCU configuration
    pub fn tcu_config(&self) -> bool {
        self.data[0] & 0x02 != 0
    }

    /// Bits 2-3 - OBD freeze frame
    pub fn obd_freeze_frame(&self) -> u8 {
        (self.data[0] >> 2) & 0x03
    }

    /// Bits 4-5 - Torque scaling (length uncertain)
    pub fn torque_scaling(&self) -> u8 {
        (self.data[0] >> 4) & 0x03
    }

    /// Bit 6 - MUL code
    pub fn mul_code(&self) -> bool {
        self.data[0] & 0x40 != 0
    }

    /// Coolant temp °C
    pub fn coolant_temp_c(&self) -> f32 {
        (self.data[1] as f32 * 0.75) - 48.0
    }

    /// Coolant temp °C x100 (integer)
    pub fn coolant_temp_c_x100(&self) -> i16 {
        (self.data[1] as i16 * 75) - 4800
    }

    /// Ambient pressure hPa (None if invalid)
    pub fn ambient_pressure_hpa(&self) -> Option<u16> {
        if self.data[2] == 0xFF {
            None
        } else {
            Some((self.data[2] as u16 * 2) + 598)
        }
    }

    /// Battery connected (0x10 = connected)
    pub fn battery_connected(&self) -> bool {
        self.data[3] == 0x10
    }

    /// Ack engine stopped
    pub fn ack_engine_stopped(&self) -> u8 {
        self.data[4]
    }

    /// Pedal position %
    pub fn pedal_position_pct(&self) -> Option<f32> {
        if self.data[5] == 0xFF {
            None
        } else {
            Some(self.data[5] as f32 * 0.390625)
        }
    }

    /// Pedal position x100
    pub fn pedal_position_x100(&self) -> Option<u16> {
        if self.data[5] == 0xFF {
            None
        } else {
            Some((self.data[5] as u16 * 25) / 64)
        }
    }

    /// Engine characteristic (bits 0-4)
    pub fn engine_characteristic(&self) -> u8 {
        self.data[6] & 0x1F
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct DME4 {
    data: [u8; 8],
}

impl DME4 {
    pub fn parse(data: [u8; 8]) -> Self {
        Self { data }
    }

    /// Bit 0 - Immobilizer authenticated
    pub fn immobilizer_authenticated(&self) -> bool {
        self.data[0] & 0x01 != 0
    }

    /// Bit 1 - MIL (check engine)
    pub fn mil_active(&self) -> bool {
        self.data[0] & 0x02 != 0
    }

    /// Bit 2 - Immobilizer enabled
    pub fn immobilizer_enabled(&self) -> bool {
        self.data[0] & 0x04 != 0
    }

    /// Bits 3-7 - Atmospheric pressure raw (unclear spec)
    pub fn atmospheric_pressure_raw(&self) -> u8 {
        (self.data[0] >> 3) & 0x1F
    }

    /// Fuel consumption (µL)
    pub fn fuel_consumption_ul(&self) -> u32 {
        let raw = u16::from_le_bytes([self.data[1], self.data[2]]);
        raw as u32 * 128
    }

    /// Fuel consumption raw (no scaling)
    pub fn fuel_consumption_raw(&self) -> u16 {
        u16::from_le_bytes([self.data[1], self.data[2]])
    }

    /// Battery voltage (V)
    pub fn battery_voltage_v(&self) -> f32 {
        self.data[3] as f32 * 0.1
    }

    /// Battery voltage (decivolts)
    pub fn battery_voltage_dv(&self) -> u8 {
        self.data[3]
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
            DME1_CAN_ID => Ok(Self::DME1(DME1::parse(*data))),
            DME2_CAN_ID => Ok(Self::DME2(DME2::parse(*data))),
            DME4_CAN_ID => Ok(Self::DME4(DME4::parse(*data))),
            _ => Err(CanBusParseError::UnknownMessageType),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dme1_parse() {
        let data = [0x05, 0xB9, 0xFC, 0x30, 0xB9, 0x06, 0x68, 0x7C];
        let dme = DME1::parse(data);

        // Bitfield
        assert_eq!(dme.swi_igk_can(), true); // bit 0
        assert_eq!(dme.f_n_eng_can(), false); // bit 1
        assert_eq!(dme.ack_tcs_can(), true); // bit 2

        // Torque
        assert_eq!(dme.tqi_tqr_can_x100(), 7215);

        // Engine speed
        assert_eq!(dme.n_eng_int(), 1959);

        // Indicated torque
        assert_eq!(dme.tqi_can_x100(), 7215);

        // Torque loss
        assert_eq!(dme.tq_loss_can_x100(), 234);

        // Vehicle speed
        assert_eq!(dme.vs(), 104);

        // Theoretical torque
        assert_eq!(dme.tqi_maf_can_x100(), 4836);
    }

    #[test]
    fn test_dme2_parse() {
        // Example: 4F B2 82 10 00 20 FF 00
        let data = [0x4F, 0xB2, 0x82, 0x10, 0x00, 0x20, 0xFF, 0x00];
        let dme = DME2::parse(data);

        // Bitfield checks
        assert_eq!(dme.can_version(), true);
        assert_eq!(dme.tcu_config(), true);
        assert_eq!(dme.obd_freeze_frame(), 0b11);
        assert_eq!(dme.torque_scaling(), 0b00);
        assert_eq!(dme.mul_code(), true);

        // Coolant temp
        assert_eq!(dme.coolant_temp_c_x100(), ((0xB2 as i16 * 75) - 4800));

        // Ambient pressure
        assert_eq!(dme.ambient_pressure_hpa(), Some((0x82 * 2 + 598) as u16));

        // Battery
        assert_eq!(dme.battery_connected(), true);

        // Engine stopped ack
        assert_eq!(dme.ack_engine_stopped(), 0x00);

        // Pedal position
        assert_eq!(dme.pedal_position_x100(), Some((0x20 * 25) / 64));

        // Engine characteristic
        assert_eq!(dme.engine_characteristic(), 0x1F); // 0xFF & 0x1F
    }

    #[test]
    fn test_dme2_invalid_values() {
        let data = [0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0x00, 0x00];
        let dme = DME2::parse(data);

        // Invalid pressure
        assert_eq!(dme.ambient_pressure_hpa(), None);

        // Invalid pedal
        assert_eq!(dme.pedal_position_pct(), None);
    }

    #[test]
    fn test_dme4_parse() {
        // Construct frame
        let data = [
            0b0000_0111, // flags
            0x34,        // FCO LSB
            0x12,        // FCO MSB
            0x96,        // 15.0V
            0,
            0,
            0,
            0,
        ];

        let dme = DME4::parse(data);

        // Bitfield
        assert_eq!(dme.immobilizer_authenticated(), true);
        assert_eq!(dme.mil_active(), true);
        assert_eq!(dme.immobilizer_enabled(), true);

        // Fuel consumption
        let raw = u16::from_le_bytes([0x34, 0x12]);
        assert_eq!(dme.fuel_consumption_raw(), raw);
        assert_eq!(dme.fuel_consumption_ul(), raw as u32 * 128);

        // Voltage
        assert_eq!(dme.battery_voltage_dv(), 0x96);
        assert_eq!(dme.battery_voltage_v(), 15.0);
    }

    #[test]
    fn test_dme4_zero_values() {
        let data = [0; 8];
        let dme = DME4::parse(data);

        assert_eq!(dme.immobilizer_authenticated(), false);
        assert_eq!(dme.mil_active(), false);
        assert_eq!(dme.fuel_consumption_ul(), 0);
        assert_eq!(dme.battery_voltage_v(), 0.0);
    }
}
