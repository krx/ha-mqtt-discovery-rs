use serde_derive::Serialize;

/// Units of measurement
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Unit {
    Power(PowerUnit),
    Volt(VoltUnit),
    Energy(EnergyUnit),
    Electrical(ElectricalUnit),
    Angle(AngleUnit),
    Currency(CurrencyUnit),
    Temperature(TempUnit),
    Time(TimeUnit),
    Length(LengthUnit),
    Frequency(FrequencyUnit),
    Pressure(PressureUnit),
    Volume(VolumeUnit),
    VolumeFlowRate(VolumeFlowRateUnit),
    Area(AreaUnit),
    Mass(MassUnit),
    Conductivity(ConductivityUnit),
    Light(LightUnit),
    Uv(UvUnit),
    Percentage(PercentageUnit),
    Irradiation(IrradiationUnit),
    Precipitation(PrecipitationUnit),
    Concentration(ConcentrationUnit),
    Speed(SpeedUnit),
    SignalStrength(SignalStrengthUnit),
    Data(DataUnit),
    DataRateUnit(DataRateUnit),
}

/// Power units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum PowerUnit {
    #[serde(rename = "W")]
    Watt,
    #[serde(rename = "kW")]
    KiloWatt,
}

/// Volt unit
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum VoltUnit {
    #[serde(rename = "V")]
    Volt,
}

/// Energy units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum EnergyUnit {
    #[serde(rename = "Wh")]
    WattHour,
    #[serde(rename = "kWh")]
    KiloWattHour,
}

/// Electrical units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ElectricalUnit {
    #[serde(rename = "A")]
    CurrentAmpere,
    #[serde(rename = "VA")]
    VoltAmpere,
}

/// Angle units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum AngleUnit {
    #[serde(rename = "°")]
    Degree,
}

/// Currency units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum CurrencyUnit {
    #[serde(rename = "€")]
    Euro,
    #[serde(rename = "$")]
    Dollar,
    #[serde(rename = "¢")]
    Cent,
}

/// Temperature units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TempUnit {
    #[serde(rename = "°C")]
    Celsius,
    #[serde(rename = "°F")]
    TempFahrenheit,
    #[serde(rename = "K")]
    TempKelvin,
}

/// Time units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TimeUnit {
    #[serde(rename = "μs")]
    Microseconds,
    #[serde(rename = "ms")]
    Milliseconds,
    #[serde(rename = "s")]
    Seconds,
    #[serde(rename = "min")]
    Minutes,
    #[serde(rename = "h")]
    Hours,
    #[serde(rename = "d")]
    Days,
    #[serde(rename = "w")]
    Weeks,
    #[serde(rename = "m")]
    Months,
    #[serde(rename = "y")]
    Years,
}

/// Length units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum LengthUnit {
    #[serde(rename = "mm")]
    Millimeters,
    #[serde(rename = "cm")]
    Centimeters,
    #[serde(rename = "m")]
    Meters,
    #[serde(rename = "km")]
    Kilometers,

    #[serde(rename = "in")]
    Inches,
    #[serde(rename = "ft")]
    Feet,
    #[serde(rename = "yd")]
    Yard,
    #[serde(rename = "mi")]
    Miles,
}

/// Frequency units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum FrequencyUnit {
    #[serde(rename = "Hz")]
    Hertz,
    #[serde(rename = "GHz")]
    GigaHertz,
}

/// Pressure units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum PressureUnit {
    #[serde(rename = "Pa")]
    Pa,
    #[serde(rename = "hPa")]
    HPa,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "mbar")]
    MBar,
    #[serde(rename = "inHg")]
    InHg,
    #[serde(rename = "psi")]
    Psi,
}

/// Volume units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum VolumeUnit {
    #[serde(rename = "L")]
    Liters,
    #[serde(rename = "mL")]
    Milliliters,
    #[serde(rename = "m³")]
    CubicMeters,
    #[serde(rename = "ft³")]
    CubicFeet,

    #[serde(rename = "gal")]
    Gallons,
    #[serde(rename = "fl. oz.")]
    FluidOunce,
}

/// Volume Flow Rate units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum VolumeFlowRateUnit {
    #[serde(rename = "m³/h")]
    CubicMetersPerHour,
    #[serde(rename = "ft³/m")]
    CubicFeetPerMinute,
}
/// Area units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum AreaUnit {
    #[serde(rename = "m²")]
    SquareMeters,
}

/// Mass units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum MassUnit {
    #[serde(rename = "g")]
    Grams,
    #[serde(rename = "kg")]
    Kilograms,
    #[serde(rename = "mg")]
    Milligrams,
    #[serde(rename = "µg")]
    Micrograms,

    #[serde(rename = "oz")]
    Ounces,
    #[serde(rename = "lb")]
    Pounds,
}

/// Conductivity units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ConductivityUnit {
    #[serde(rename = "µS/cm")]
    Conductivity,
}

/// Light units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum LightUnit {
    #[serde(rename = "lx")]
    Lux,
}

/// UV Index units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum UvUnit {
    #[serde(rename = "UV index")]
    UvIndex,
}

/// Percentage units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum PercentageUnit {
    #[serde(rename = "%")]
    Percentage,
}

/// Irradiation units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum IrradiationUnit {
    #[serde(rename = "W/m²")]
    WattsPerSquareMeter,
}

/// Precipitation units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum PrecipitationUnit {
    #[serde(rename = "mm/h")]
    MillimetersPerHour,
}

/// Concentration units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ConcentrationUnit {
    #[serde(rename = "µg/m³")]
    MicrogramsPerCubicMeter,
    #[serde(rename = "mg/m³")]
    MilligramsPerCubicMeter,
    #[serde(rename = "p/m³")]
    PartsPerCubicMeter,
    #[serde(rename = "ppm")]
    PartsPerMillion,
    #[serde(rename = "ppb")]
    PartsPerBillion,
}

/// Speed units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SpeedUnit {
    #[serde(rename = "mm/d")]
    MillimetersPerDay,
    #[serde(rename = "in/d")]
    InchesPerDay,
    #[serde(rename = "m/s")]
    MetersPerSecond,
    #[serde(rename = "in/h")]
    InchesPerHour,
    #[serde(rename = "km/h")]
    KilometersPerHour,
    #[serde(rename = "mph")]
    MilesPerHour,
}

/// Signal_strength units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SignalStrengthUnit {
    #[serde(rename = "dB")]
    Decibels,
    #[serde(rename = "dBm")]
    DecibelsMilliwatt,
}

/// Data units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum DataUnit {
    #[serde(rename = "bit")]
    Bits,
    #[serde(rename = "kbit")]
    Kilobits,
    #[serde(rename = "Mbit")]
    Megabits,
    #[serde(rename = "Gbit")]
    Gigabits,
    #[serde(rename = "B")]
    Bytes,
    #[serde(rename = "kB")]
    Kilobytes,
    #[serde(rename = "MB")]
    Megabytes,
    #[serde(rename = "GB")]
    Gigabytes,
    #[serde(rename = "TB")]
    Terabytes,
    #[serde(rename = "PB")]
    Petabytes,
    #[serde(rename = "EB")]
    Exabytes,
    #[serde(rename = "ZB")]
    Zettabytes,
    #[serde(rename = "YB")]
    Yottabytes,
    #[serde(rename = "KiB")]
    Kibibytes,
    #[serde(rename = "MiB")]
    Mebibytes,
    #[serde(rename = "GiB")]
    Gibibytes,
    #[serde(rename = "TiB")]
    Tebibytes,
    #[serde(rename = "PiB")]
    Pebibytes,
    #[serde(rename = "EiB")]
    Exbibytes,
    #[serde(rename = "ZiB")]
    Zebibytes,
    #[serde(rename = "YiB")]
    Yobibytes,
}

/// Data rate units
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum DataRateUnit {
    #[serde(rename = "bit/s")]
    BitsPerSecond,
    #[serde(rename = "kbit/s")]
    KilobitsPerSecond,
    #[serde(rename = "Mbit/s")]
    MegabitsPerSecond,
    #[serde(rename = "Gbit/s")]
    GigabitsPerSecond,
    #[serde(rename = "B/s")]
    BytesPerSecond,
    #[serde(rename = "kB/s")]
    KilobytesPerSecond,
    #[serde(rename = "MB/s")]
    MegabytesPerSecond,
    #[serde(rename = "GB/s")]
    GigabytesPerSecond,
    #[serde(rename = "KiB/s")]
    KibibytesPerSecond,
    #[serde(rename = "MiB/s")]
    MebibytesPerSecond,
    #[serde(rename = "GiB/s")]
    GibibytesPerSecond,
}
