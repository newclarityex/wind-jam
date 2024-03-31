//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    pub struct GKTurnBasedMatchmakerViewController;

    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    unsafe impl ClassType for GKTurnBasedMatchmakerViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl GKViewController for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl NSCoding for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl NSEditor for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl NSObjectProtocol for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl NSSeguePerforming for GKTurnBasedMatchmakerViewController {}

#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
unsafe impl NSUserInterfaceItemIdentification for GKTurnBasedMatchmakerViewController {}

extern_methods!(
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    unsafe impl GKTurnBasedMatchmakerViewController {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    unsafe impl GKTurnBasedMatchmakerViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    unsafe impl GKTurnBasedMatchmakerViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    unsafe impl GKTurnBasedMatchmakerViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
    unsafe impl GKTurnBasedMatchmakerViewController {
        #[method_id(@__retain_semantics Other turnBasedMatchmakerDelegate)]
        pub unsafe fn turnBasedMatchmakerDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn GKTurnBasedMatchmakerViewControllerDelegate>>>;

        #[method(setTurnBasedMatchmakerDelegate:)]
        pub unsafe fn setTurnBasedMatchmakerDelegate(
            &self,
            turn_based_matchmaker_delegate: Option<
                &ProtocolObject<dyn GKTurnBasedMatchmakerViewControllerDelegate>,
            >,
        );

        #[method(showExistingMatches)]
        pub unsafe fn showExistingMatches(&self) -> bool;

        #[method(setShowExistingMatches:)]
        pub unsafe fn setShowExistingMatches(&self, show_existing_matches: bool);

        #[method(matchmakingMode)]
        pub unsafe fn matchmakingMode(&self) -> GKMatchmakingMode;

        #[method(setMatchmakingMode:)]
        pub unsafe fn setMatchmakingMode(&self, matchmaking_mode: GKMatchmakingMode);

        #[cfg(feature = "GameKit_GKMatchRequest")]
        #[method_id(@__retain_semantics Init initWithMatchRequest:)]
        pub unsafe fn initWithMatchRequest(
            this: Option<Allocated<Self>>,
            request: &GKMatchRequest,
        ) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait GKTurnBasedMatchmakerViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
        #[method(turnBasedMatchmakerViewControllerWasCancelled:)]
        unsafe fn turnBasedMatchmakerViewControllerWasCancelled(
            &self,
            view_controller: &GKTurnBasedMatchmakerViewController,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "GameKit_GKTurnBasedMatchmakerViewController"
        ))]
        #[method(turnBasedMatchmakerViewController:didFailWithError:)]
        unsafe fn turnBasedMatchmakerViewController_didFailWithError(
            &self,
            view_controller: &GKTurnBasedMatchmakerViewController,
            error: &NSError,
        );

        #[cfg(all(
            feature = "GameKit_GKTurnBasedMatch",
            feature = "GameKit_GKTurnBasedMatchmakerViewController"
        ))]
        #[deprecated = "use GKTurnBasedEventListener player:receivedTurnEventForMatch:didBecomeActive:"]
        #[optional]
        #[method(turnBasedMatchmakerViewController:didFindMatch:)]
        unsafe fn turnBasedMatchmakerViewController_didFindMatch(
            &self,
            view_controller: &GKTurnBasedMatchmakerViewController,
            r#match: &GKTurnBasedMatch,
        );

        #[cfg(all(
            feature = "GameKit_GKTurnBasedMatch",
            feature = "GameKit_GKTurnBasedMatchmakerViewController"
        ))]
        #[deprecated = "use GKTurnBasedEventListener player:wantsToQuitMatch:"]
        #[optional]
        #[method(turnBasedMatchmakerViewController:playerQuitForMatch:)]
        unsafe fn turnBasedMatchmakerViewController_playerQuitForMatch(
            &self,
            view_controller: &GKTurnBasedMatchmakerViewController,
            r#match: &GKTurnBasedMatch,
        );
    }

    unsafe impl ProtocolType for dyn GKTurnBasedMatchmakerViewControllerDelegate {}
);
