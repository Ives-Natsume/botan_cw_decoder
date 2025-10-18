use std::fmt;

/// Represents the parsed BOTAN beacon data
#[derive(Debug, Clone)]
pub struct BotanBeaconData {
    pub satellite_name: String,
    pub call_sign: String,
    pub rssi: Option<BotanRSSI>,
    pub telemetry: BotanTelemetry,
}

/// Represents RSSI information if available
#[derive(Debug, Clone)]
pub struct BotanRSSI {
    pub rssi_dbm: f64,      // RSSI in dBm
    pub snr_db: f64,        // Signal-to-Noise Ratio in dB
}

/// Represents the 8-byte telemetry data block
#[derive(Debug, Clone)]
pub struct BotanTelemetry {
    pub bat_v: f64,        // Battery Voltage [V]
    pub bat_i: f64,        // Battery Current [mA]  
    pub bat_t: f64,        // Battery Temperature [°C]
    pub bpb_t: f64,        // Circuit board Temperature [°C]
    pub raw_i: f64,        // Current Consumption [mA]
    pub data1: Data1Flags, // Power system status
    pub data2: Data2Flags, // Command counters and KILL switch
    pub data3: Data3Flags, // Mission status flags
}

/// Bitfield for data1 (Byte 6) - Power system status
#[derive(Debug, Clone)]
pub struct Data1Flags {
    pub power_5v0: bool,      // Bit 7: 5V PWR Line On/Off
    pub power_depant: bool,   // Bit 6: Antenna Deployment PWR Line On/Off
    pub power_com: bool,      // Bit 5: Transponder PWR Line On/Off
    pub sap_x_pos: bool,      // Bit 4: +X PWR generation
    pub sap_y_pos: bool,      // Bit 3: +Y PWR generation
    pub sap_y_neg: bool,      // Bit 2: -Y PWR generation
    pub sap_z_pos: bool,      // Bit 1: +Z PWR generation
    pub sap_z_neg: bool,      // Bit 0: -Z PWR generation
}

/// Bitfield for data2 (Byte 7) - Command counters and KILL switch
#[derive(Debug, Clone)]
pub struct Data2Flags {
    pub reserve_cmd_counter: u8, // Bits 7-4: Reserved commands count
    pub cmd_uplink_counter: u8,  // Bits 3-1: Received commands count
    pub kill_sw: bool,           // Bit 0: KILL Switch status
}

/// Bitfield for data3 (Byte 8) - Mission status
#[derive(Debug, Clone)]
pub struct Data3Flags {
    pub kill_counter: u8,        // Bits 7-6: KILL SW occurrences count
    pub mission_pic_on: bool,    // Bit 5: Mission PIC On/Off
    pub mis_error_flag: bool,    // Bit 4: Mission Error Flag
    pub mis_end_flag: bool,      // Bit 3: Mission END Flag
    pub aprs_flag: bool,         // Bit 2: APRS Mission execution Flag
    pub current_mis: u8,         // Bits 1-0: Current Mission (00:None, 01:Earth, 10:Sun)
}

impl fmt::Display for BotanBeaconData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "BOTAN Satellite Beacon Data")?;
        writeln!(f, "==========================")?;
        writeln!(f, "Satellite: {}", self.satellite_name)?;
        writeln!(f, "Call Sign: {}", self.call_sign)?;
        writeln!(f)?;
        writeln!(f, "Signal Information (lack definition):")?;
        writeln!(f, "--------------")?;
        if let Some(rssi) = &self.rssi {
            writeln!(f, "Unknown argument 1: {:.1}", rssi.rssi_dbm)?;
            writeln!(f, "Unknown argument 2: {:.1}", rssi.snr_db)?;
        } else {
            writeln!(f, "Unknown argument 1: N/A")?;
            writeln!(f, "Unknown argument 2: N/A")?;
        }
        writeln!(f)?;
        writeln!(f, "Telemetry Data:")?;
        writeln!(f, "--------------")?;
        writeln!(f, "Battery Voltage:      {:.3} V", self.telemetry.bat_v)?;
        writeln!(f, "Battery Current:      {:.1} mA", self.telemetry.bat_i)?;
        writeln!(f, "Battery Temperature:  {:.1} °C", self.telemetry.bat_t)?;
        writeln!(f, "Board Temperature:    {:.1} °C", self.telemetry.bpb_t)?;
        writeln!(f, "Current Consumption:  {:.1} mA", self.telemetry.raw_i)?;
        writeln!(f)?;
        writeln!(f, "Power System Status:")?;
        writeln!(f, "  5V Power Line:      {}", if self.telemetry.data1.power_5v0 { "ON" } else { "OFF" })?;
        writeln!(f, "  Antenna Deployment: {}", if self.telemetry.data1.power_depant { "ON" } else { "OFF" })?;
        writeln!(f, "  Transponder:        {}", if self.telemetry.data1.power_com { "ON" } else { "OFF" })?;
        writeln!(f, "  Solar Panels:")?;
        writeln!(f, "    +X: {} | +Y: {} | -Y: {} | +Z: {} | -Z: {}", 
                 if self.telemetry.data1.sap_x_pos { "ON" } else { "OFF" },
                 if self.telemetry.data1.sap_y_pos { "ON" } else { "OFF" },
                 if self.telemetry.data1.sap_y_neg { "ON" } else { "OFF" },
                 if self.telemetry.data1.sap_z_pos { "ON" } else { "OFF" },
                 if self.telemetry.data1.sap_z_neg { "ON" } else { "OFF" })?;
        writeln!(f)?;
        writeln!(f, "Command Status:")?;
        writeln!(f, "  Reserved Commands:  {}", self.telemetry.data2.reserve_cmd_counter)?;
        writeln!(f, "  Uplink Commands:    {}", self.telemetry.data2.cmd_uplink_counter)?;
        writeln!(f, "  KILL Switch:        {}", if self.telemetry.data2.kill_sw { "ON" } else { "OFF" })?;
        writeln!(f)?;
        writeln!(f, "Mission Status:")?;
        writeln!(f, "  KILL Counter:       {}", self.telemetry.data3.kill_counter)?;
        writeln!(f, "  Mission PIC:        {}", if self.telemetry.data3.mission_pic_on { "ON" } else { "OFF" })?;
        writeln!(f, "  Mission Error:      {}", if self.telemetry.data3.mis_error_flag { "YES" } else { "NO" })?;
        writeln!(f, "  Mission End:        {}", if self.telemetry.data3.mis_end_flag { "YES" } else { "NO" })?;
        writeln!(f, "  APRS Mission:       {}", if self.telemetry.data3.aprs_flag { "ACTIVE" } else { "INACTIVE" })?;
        writeln!(f, "  Current Mission:    {}", match self.telemetry.data3.current_mis {
            0 => "None",
            1 => "Earth",
            2 => "Sun", 
            _ => "Unknown"
        })
    }
}

/// Parse a BOTAN beacon string
pub fn parse_botan_beacon(input: &str) -> Result<BotanBeaconData, String> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    if parts.len() < 3 {
        return Err("Invalid beacon format. Expected: BOTAN JS1YPT (Optional<RSSI>) <data>".to_string());
    }
    
    // Validate header
    if parts[0] != "BOTAN" {
        return Err(format!("Invalid satellite name. Expected 'BOTAN', got '{}'", parts[0]));
    }
    
    if parts[1] != "JS1YPT" {
        return Err(format!("Invalid call sign. Expected 'JS1YPT', got '{}'", parts[1]));
    }

    // Check for optional RSSI info
    // Format: SI<HEX data>     e.g., "SI8640"
    let rssi = if parts.len() == 4 {
        let rssi_str = parts[2];
        if !rssi_str.starts_with("SI") || rssi_str.len() != 6 {
            return Err("Invalid RSSI format. Expected 'SI' followed by 4 hex characters".to_string());
        }
        // Parse RSSI & SNR from hex
        let rssi_hex = &rssi_str[2..4];
        let snr_hex = &rssi_str[4..];
        let rssi_dbm = match u8::from_str_radix(rssi_hex, 16) {
            Ok(val) => val as f64,
            Err(_) => return Err("Invalid RSSI hex value".to_string()),
        };
        let snr_db = match u8::from_str_radix(snr_hex, 16) {
            Ok(val) => val as f64,
            Err(_) => return Err("Invalid SNR hex value".to_string()),
        };
        Some(BotanRSSI { rssi_dbm, snr_db })
    } else {
        None
    };
    
    // Parse the 8-byte data block
    let data_str = match rssi {
        Some(_) => parts[3],
        None => parts[2],
    };
    if data_str.len() != 16 { // 8 bytes = 16 hex characters
        return Err(format!("Invalid data length. Expected 16 hex characters, got {}", data_str.len()));
    }
    
    // Convert hex string to bytes
    let mut bytes = Vec::new();
    for i in (0..data_str.len()).step_by(2) {
        match u8::from_str_radix(&data_str[i..i+2], 16) {
            Ok(byte) => bytes.push(byte),
            Err(_) => return Err(format!("Invalid hex data at position {}-{}: {}", i, i+1, &data_str[i..i+2])),
        }
    }
    
    if bytes.len() != 8 {
        return Err(format!("Expected 8 bytes, got {}", bytes.len()));
    }
    
    // Parse telemetry according to the definition
    let telemetry = parse_telemetry_bytes(&bytes)?;
    
    Ok(BotanBeaconData {
        satellite_name: "BOTAN".to_string(),
        call_sign: "JS1YPT".to_string(),
        rssi,
        telemetry,
    })
}

fn parse_telemetry_bytes(bytes: &[u8]) -> Result<BotanTelemetry, String> {
    if bytes.len() != 8 {
        return Err(format!("Expected 8 bytes for telemetry, got {}", bytes.len()));
    }
    
    // Convert bytes to decimal values for calculations
    let byte1 = bytes[0] as f64; // BAT_V
    let byte2 = bytes[1] as f64; // BAT_I  
    let byte3 = bytes[2] as f64; // BAT_T
    let byte4 = bytes[3] as f64; // BPB_T
    let byte5 = bytes[4] as f64; // RAW_I
    let byte6 = bytes[5];        // data1 (bitfield)
    let byte7 = bytes[6];        // data2 (bitfield)
    let byte8 = bytes[7];        // data3 (bitfield)
    
    // Calculate converted values according to formulas in definition
    let bat_v = byte1 * 0.025781;
    let bat_i = byte2 * (-50.045) + 6330.4;
    
    // Battery temperature calculation (complex formula)
    let bat_t = {
        let inner = (byte3 * 0.01289) / (3.3 - byte3 * 0.01289);
        if inner <= 0.0 {
            return Err("Invalid battery temperature calculation: logarithm of non-positive number".to_string());
        }
        (1185000.0 / (inner.ln() * 298.0 + 3976.0)) - 273.0
    };
    
    // Board temperature calculation
    let bpb_t = {
        let discriminant = 36.44506 - byte4 * 0.06875;
        if discriminant < 0.0 {
            return Err("Invalid board temperature calculation: square root of negative number".to_string());
        }
        30.0 - ((discriminant.sqrt() - 5.506) / 0.00352)
    };
    
    let raw_i = byte5 * 51.84 - 1950.9;
    
    // Parse bitfields
    let data1 = Data1Flags {
        power_5v0: (byte6 & 0x80) != 0,      // Bit 7
        power_depant: (byte6 & 0x40) != 0,   // Bit 6  
        power_com: (byte6 & 0x20) != 0,      // Bit 5
        sap_x_pos: (byte6 & 0x10) != 0,      // Bit 4
        sap_y_pos: (byte6 & 0x08) != 0,      // Bit 3
        sap_y_neg: (byte6 & 0x04) != 0,      // Bit 2
        sap_z_pos: (byte6 & 0x02) != 0,      // Bit 1
        sap_z_neg: (byte6 & 0x01) != 0,      // Bit 0
    };
    
    let data2 = Data2Flags {
        reserve_cmd_counter: (byte7 >> 4) & 0x07,  // Bits 7-4
        cmd_uplink_counter: (byte7 >> 1) & 0x07,   // Bits 3-1
        kill_sw: (byte7 & 0x01) != 0,              // Bit 0
    };
    
    let data3 = Data3Flags {
        kill_counter: (byte8 >> 6) & 0x03,         // Bits 7-6
        mission_pic_on: (byte8 & 0x20) != 0,       // Bit 5
        mis_error_flag: (byte8 & 0x10) != 0,       // Bit 4
        mis_end_flag: (byte8 & 0x08) != 0,         // Bit 3
        aprs_flag: (byte8 & 0x04) != 0,            // Bit 2
        current_mis: byte8 & 0x03,                 // Bits 1-0
    };
    
    Ok(BotanTelemetry {
        bat_v,
        bat_i,
        bat_t,
        bpb_t,
        raw_i,
        data1,
        data2,
        data3,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_beacon() {
        let input = "BOTAN JS1YPT A57EB76823210E08";
        let result = parse_botan_beacon(input);
        assert!(result.is_ok());
        
        let beacon = result.unwrap();
        assert_eq!(beacon.satellite_name, "BOTAN");
        assert_eq!(beacon.call_sign, "JS1YPT");
    }

    #[test]
    fn test_parse_invalid_header() {
        let input = "WRONG JS1YPT A57EB76823210E08";
        let result = parse_botan_beacon(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_data_length() {
        let input = "BOTAN JS1YPT A57EB768";  // Too short
        let result = parse_botan_beacon(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_hex() {
        let input = "BOTAN JS1YPT G57EB76823210E08";  // 'G' is not valid hex
        let result = parse_botan_beacon(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_example_beacon() {
        let input = "BOTAN JS1YPT A57EB76823210E08";
        let result = parse_botan_beacon(input);
        assert!(result.is_ok());
        
        let beacon = result.unwrap();
        assert_eq!(beacon.satellite_name, "BOTAN");
        assert_eq!(beacon.call_sign, "JS1YPT");
        
        // Check some basic calculations
        // Byte 1 (A5 = 165): BAT_V = 165 * 0.025781 ≈ 4.254 V
        assert!((beacon.telemetry.bat_v - 4.254).abs() < 0.01);
        
        // The exact values depend on the formulas, but we can check they're reasonable
        println!("Parsed beacon data:\n{}", beacon);
    }

    #[test]
    fn test_different_beacon_values() {
        // Test with safe values that won't cause math errors
        let input = "BOTAN JS1YPT 5050505050505050";
        let result = parse_botan_beacon(input);
        assert!(result.is_ok());
        
        if let Ok(beacon) = result {
            println!("Mid-range values beacon:\n{}", beacon);
        }
        
        // Test with all zeros - may fail due to temperature calculations
        let input2 = "BOTAN JS1YPT 0000000000000000";
        let result2 = parse_botan_beacon(input2);
        // This might fail due to logarithm of 0, which is expected behavior
        if result2.is_err() {
            println!("Zero values correctly failed due to math constraints");
        }
    }
}