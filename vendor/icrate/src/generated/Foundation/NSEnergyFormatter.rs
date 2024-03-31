//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSEnergyFormatterUnit {
        NSEnergyFormatterUnitJoule = 11,
        NSEnergyFormatterUnitKilojoule = 14,
        NSEnergyFormatterUnitCalorie = (7 << 8) + 1,
        NSEnergyFormatterUnitKilocalorie = (7 << 8) + 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSEnergyFormatter")]
    pub struct NSEnergyFormatter;

    #[cfg(feature = "Foundation_NSEnergyFormatter")]
    unsafe impl ClassType for NSEnergyFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSEnergyFormatter")]
unsafe impl NSCoding for NSEnergyFormatter {}

#[cfg(feature = "Foundation_NSEnergyFormatter")]
unsafe impl NSCopying for NSEnergyFormatter {}

#[cfg(feature = "Foundation_NSEnergyFormatter")]
unsafe impl NSObjectProtocol for NSEnergyFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSEnergyFormatter")]
    unsafe impl NSEnergyFormatter {
        #[cfg(feature = "Foundation_NSNumberFormatter")]
        #[method_id(@__retain_semantics Other numberFormatter)]
        pub unsafe fn numberFormatter(&self) -> Id<NSNumberFormatter>;

        #[cfg(feature = "Foundation_NSNumberFormatter")]
        #[method(setNumberFormatter:)]
        pub unsafe fn setNumberFormatter(&self, number_formatter: Option<&NSNumberFormatter>);

        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle;

        #[method(setUnitStyle:)]
        pub unsafe fn setUnitStyle(&self, unit_style: NSFormattingUnitStyle);

        #[method(isForFoodEnergyUse)]
        pub unsafe fn isForFoodEnergyUse(&self) -> bool;

        #[method(setForFoodEnergyUse:)]
        pub unsafe fn setForFoodEnergyUse(&self, for_food_energy_use: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromValue:unit:)]
        pub unsafe fn stringFromValue_unit(
            &self,
            value: c_double,
            unit: NSEnergyFormatterUnit,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringFromJoules:)]
        pub unsafe fn stringFromJoules(&self, number_in_joules: c_double) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitStringFromValue:unit:)]
        pub unsafe fn unitStringFromValue_unit(
            &self,
            value: c_double,
            unit: NSEnergyFormatterUnit,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unitStringFromJoules:usedUnit:)]
        pub unsafe fn unitStringFromJoules_usedUnit(
            &self,
            number_in_joules: c_double,
            unitp: *mut NSEnergyFormatterUnit,
        ) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<AnyObject>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString>>>,
        ) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSEnergyFormatter")]
    unsafe impl NSEnergyFormatter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
