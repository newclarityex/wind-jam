//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "CLAvailability.rs"]
mod __CLAvailability;
#[path = "CLBeaconIdentityConstraint.rs"]
mod __CLBeaconIdentityConstraint;
#[path = "CLBeaconRegion.rs"]
mod __CLBeaconRegion;
#[path = "CLCircularRegion.rs"]
mod __CLCircularRegion;
#[path = "CLError.rs"]
mod __CLError;
#[path = "CLErrorDomain.rs"]
mod __CLErrorDomain;
#[path = "CLGeocoder.rs"]
mod __CLGeocoder;
#[path = "CLHeading.rs"]
mod __CLHeading;
#[path = "CLLocation.rs"]
mod __CLLocation;
#[path = "CLLocationManager.rs"]
mod __CLLocationManager;
#[path = "CLLocationManagerDelegate.rs"]
mod __CLLocationManagerDelegate;
#[path = "CLLocationManager_CLVisitExtensions.rs"]
mod __CLLocationManager_CLVisitExtensions;
#[path = "CLLocationPushServiceError.rs"]
mod __CLLocationPushServiceError;
#[path = "CLLocationPushServiceExtension.rs"]
mod __CLLocationPushServiceExtension;
#[path = "CLPlacemark.rs"]
mod __CLPlacemark;
#[path = "CLRegion.rs"]
mod __CLRegion;
#[path = "CLVisit.rs"]
mod __CLVisit;

#[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
pub use self::__CLBeaconIdentityConstraint::CLBeaconIdentityConstraint;
pub use self::__CLBeaconIdentityConstraint::CLBeaconMajorValue;
pub use self::__CLBeaconIdentityConstraint::CLBeaconMinorValue;
#[cfg(feature = "CoreLocation_CLBeacon")]
pub use self::__CLBeaconRegion::CLBeacon;
#[cfg(feature = "CoreLocation_CLBeaconRegion")]
pub use self::__CLBeaconRegion::CLBeaconRegion;
#[cfg(feature = "CoreLocation_CLCircularRegion")]
pub use self::__CLCircularRegion::CLCircularRegion;
pub use self::__CLError::kCLErrorUserInfoAlternateRegionKey;
pub use self::__CLError::{
    kCLErrorDeferredAccuracyTooLow, kCLErrorDeferredCanceled, kCLErrorDeferredDistanceFiltered,
    kCLErrorDeferredFailed, kCLErrorDeferredNotUpdatingLocation, kCLErrorDenied,
    kCLErrorGeocodeCanceled, kCLErrorGeocodeFoundNoResult, kCLErrorGeocodeFoundPartialResult,
    kCLErrorHeadingFailure, kCLErrorHistoricalLocationError, kCLErrorLocationUnknown,
    kCLErrorNetwork, kCLErrorPromptDeclined, kCLErrorRangingFailure, kCLErrorRangingUnavailable,
    kCLErrorRegionMonitoringDenied, kCLErrorRegionMonitoringFailure,
    kCLErrorRegionMonitoringResponseDelayed, kCLErrorRegionMonitoringSetupDelayed, CLError,
};
pub use self::__CLErrorDomain::kCLErrorDomain;
pub use self::__CLGeocoder::CLGeocodeCompletionHandler;
#[cfg(feature = "CoreLocation_CLGeocoder")]
pub use self::__CLGeocoder::CLGeocoder;
pub use self::__CLHeading::kCLHeadingFilterNone;
#[cfg(feature = "CoreLocation_CLHeading")]
pub use self::__CLHeading::CLHeading;
pub use self::__CLHeading::CLHeadingComponentValue;
pub use self::__CLLocation::kCLDistanceFilterNone;
pub use self::__CLLocation::kCLLocationAccuracyBest;
pub use self::__CLLocation::kCLLocationAccuracyBestForNavigation;
pub use self::__CLLocation::kCLLocationAccuracyHundredMeters;
pub use self::__CLLocation::kCLLocationAccuracyKilometer;
pub use self::__CLLocation::kCLLocationAccuracyNearestTenMeters;
pub use self::__CLLocation::kCLLocationAccuracyReduced;
pub use self::__CLLocation::kCLLocationAccuracyThreeKilometers;
pub use self::__CLLocation::kCLLocationCoordinate2DInvalid;
#[cfg(feature = "CoreLocation_CLFloor")]
pub use self::__CLLocation::CLFloor;
#[cfg(feature = "CoreLocation_CLLocation")]
pub use self::__CLLocation::CLLocation;
pub use self::__CLLocation::CLLocationAccuracy;
pub use self::__CLLocation::CLLocationCoordinate2D;
pub use self::__CLLocation::CLLocationCoordinate2DIsValid;
pub use self::__CLLocation::CLLocationCoordinate2DMake;
pub use self::__CLLocation::CLLocationDegrees;
pub use self::__CLLocation::CLLocationDirection;
pub use self::__CLLocation::CLLocationDirectionAccuracy;
pub use self::__CLLocation::CLLocationDistance;
pub use self::__CLLocation::CLLocationDistanceMax;
#[cfg(feature = "CoreLocation_CLLocationSourceInformation")]
pub use self::__CLLocation::CLLocationSourceInformation;
pub use self::__CLLocation::CLLocationSpeed;
pub use self::__CLLocation::CLLocationSpeedAccuracy;
pub use self::__CLLocation::CLTimeIntervalMax;
#[cfg(feature = "CoreLocation_CLLocationManager")]
pub use self::__CLLocationManager::CLLocationManager;
pub use self::__CLLocationManager::{
    kCLAuthorizationStatusAuthorized, kCLAuthorizationStatusAuthorizedAlways,
    kCLAuthorizationStatusAuthorizedWhenInUse, kCLAuthorizationStatusDenied,
    kCLAuthorizationStatusNotDetermined, kCLAuthorizationStatusRestricted, CLAuthorizationStatus,
};
pub use self::__CLLocationManager::{
    CLAccuracyAuthorization, CLAccuracyAuthorizationFullAccuracy,
    CLAccuracyAuthorizationReducedAccuracy,
};
pub use self::__CLLocationManager::{
    CLActivityType, CLActivityTypeAirborne, CLActivityTypeAutomotiveNavigation,
    CLActivityTypeFitness, CLActivityTypeOther, CLActivityTypeOtherNavigation,
};
pub use self::__CLLocationManager::{
    CLDeviceOrientation, CLDeviceOrientationFaceDown, CLDeviceOrientationFaceUp,
    CLDeviceOrientationLandscapeLeft, CLDeviceOrientationLandscapeRight,
    CLDeviceOrientationPortrait, CLDeviceOrientationPortraitUpsideDown, CLDeviceOrientationUnknown,
};
pub use self::__CLLocationManagerDelegate::CLLocationManagerDelegate;
pub use self::__CLLocationPushServiceError::CLLocationPushServiceErrorDomain;
pub use self::__CLLocationPushServiceError::{
    CLLocationPushServiceError, CLLocationPushServiceErrorMissingEntitlement,
    CLLocationPushServiceErrorMissingPushExtension,
    CLLocationPushServiceErrorMissingPushServerEnvironment, CLLocationPushServiceErrorUnknown,
};
pub use self::__CLLocationPushServiceExtension::CLLocationPushServiceExtension;
#[cfg(feature = "CoreLocation_CLPlacemark")]
pub use self::__CLPlacemark::CLPlacemark;
#[cfg(feature = "CoreLocation_CLRegion")]
pub use self::__CLRegion::CLRegion;
pub use self::__CLRegion::{
    CLProximity, CLProximityFar, CLProximityImmediate, CLProximityNear, CLProximityUnknown,
};
pub use self::__CLRegion::{
    CLRegionState, CLRegionStateInside, CLRegionStateOutside, CLRegionStateUnknown,
};
#[cfg(feature = "CoreLocation_CLVisit")]
pub use self::__CLVisit::CLVisit;
