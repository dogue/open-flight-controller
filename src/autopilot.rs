//! Provides a convenient API for getting and setting various values and modes of the autopilot.

/// Toggleable modes for the autopilot.
pub enum Modes {
    LateralNavigation   = 0,

    /// AUTOPILOT_HEADING_LOCK
    HeadingHold         = 1,

    /// AUTOPILOT_ALTITUDE_HOLD
    AltitudeHold        = 2,

    /// AUTOPILOT_VERTICAL_HOLD
    VerticalSpeedHold   = 3,

    /// AUTOPILOT_MANAGED_SPEED_IN_MACH
    SpeedUnitsMach      = 4,

    /// AUTOPILOT_MASTER
    AutopilotMaster     = 5,

    /// AUTOPILOT_YAW_DAMPER
    YawDamper           = 6,

    /// AUTOPILOT_FLIGHT_LEVEL_CHANGE
    FlightLevelChange   = 7,

    VerticalNavigation  = 8,

    /// AUTOPILOT_MACH_HOLD
    AirspeedHold        = 9,

    /// AUTOPILOT_THROTTLE_ARM
    Autothrottle        = 10,

    /// AUTOPILOT_BACKCOURSE_HOLD
    Backcourse          = 11
}

pub struct Autopilot {
    
}