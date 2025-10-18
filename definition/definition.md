# BOTAN CW Beacon Format Definition

## 1. Beacon Structure

The CW beacon transmits a static header followed by an 8-byte data block.

* **Satellite Name:** `BOTAN`
* **Call Sign:** `JS1YPT`
* **Data:** 8-byte telemetry block (defined in Section 2).

The full beacon message is: `BOTAN` `JS1YPT` `[Data]`

---

## 2. CW Data Format (8 Bytes)

This 8-byte block contains satellite telemetry. The value used in the conversion formulas is the decimal (DEC) representation of the raw byte.

| Byte | Identification | Description | Conversion Formula (Value = DEC) |
| :--- | :--- | :--- | :--- |
| 1 | `BAT_V` | Battery Voltage | `BAT_V[V] = Value \times 0.025781` |
| 2 | `BAT_I` | Battery Current | `BAT_I[mA] = Value \times (-50.045) + 6330.4` |
| 3 | `BAT_T` | Battery Temperature | $BAT\_T[^{\circ}C] = \left( \frac{1185000}{\ln\left(\frac{\text{Value} \times 0.01289}{3.3 - \text{Value} \times 0.01289}\right)} \times 298 + 3976 \right) - 273$ |
| 4 | `BPB_T` | Circuit board Temperature | $BPB\_T[^{\circ}C] = 30 - \frac{\sqrt{36.44506 - \text{Value} \times 0.06875 - 5.506}}{0.00352}$ |
| 5 | `RAW_I` | Current Consumption | `RAW_I[mA] = Value \times 51.84 - 1950.9` |
| 6 | `data1` | See Table "data1" | N/A (Bitfield) |
| 7 | `data2` | See Table "data2" | N/A (Bitfield) |
| 8 | `data3` | See Table "data3" | N/A (Bitfield) |

---

## 3. Bitfield Definitions

### 3.1. data1 (Byte 6)

This byte contains power system status flags.

| Bit | Identification | Description |
| :-- | :--- | :--- |
| 7 | `Power_5V0` | 5V PWR Line On/Off Flag. |
| 6 | `Power_DEPANT` | Antenna Deployment PWR Line On/Off Flag. |
| 5 | `Power_COM` | Transponder PWR Line On/Off Flag. |
| 4 | `SAP-X` | +X PWR generation Flag. |
| 3 | `SAP+Y` | +Y PWR generation Flag. |
| 2 | `SAP-Y` | -Y PWR generation Flag. |
| 1 | `SAP+Z` | +Z PWR generation Flag. |
| 0 | `SAP-Z` | -Z PWR generation Flag. |

### 3.2. data2 (Byte 7)

This byte contains command counters and the KILL switch status.

| Bit(s) | Identification | Description |
| :--- | :--- | :--- |
| 7 | (RESERVED) | (RESERVED) |
| 6-4 | `RESERVE_CMD_COUNTER` | Number of reserved commands. LSB first. |
| 3-1 | `CMD_UPLINK_COUNTER` | Number of received commands. LSB first. |
| 0 | `KILL_SW` | KILL Switch ON/OFF flag. |

### 3.3. data3 (Byte 8)

This byte contains mission status flags and counters.

| Bit(s) | Identification | Description |
| :--- | :--- | :--- |
| 7-6 | `KILL_COUNTER` | Counts of "KILL SW" occurrences. LSB first. |
| 5 | `MISSION_PIC_ON/OFF` | Mission PIC On/Off Flag. |
| 4 | `MIS_ERROR_FLAG` | Mission Error Flag. |
| 3 | `MIS_END_FLAG` | Mission END Flag. |
| 2 | `APRS_FLAG` | APRS Mission execution Flag. |
| 1-0 | `CURRENT_MIS` | Ongoing Mission (00:None, 01:Earth, 10:Sun) |