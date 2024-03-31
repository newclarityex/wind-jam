//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSAppKitVersionNumberWithContinuousScrollingBrowser: NSAppKitVersion = 680.0);

extern_static!(NSAppKitVersionNumberWithColumnResizingBrowser: NSAppKitVersion = 685.0);

pub type NSBrowserColumnsAutosaveName = NSString;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBrowserColumnResizingType {
        NSBrowserNoColumnResizing = 0,
        NSBrowserAutoColumnResizing = 1,
        NSBrowserUserColumnResizing = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBrowserDropOperation {
        NSBrowserDropOn = 0,
        NSBrowserDropAbove = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSBrowser")]
    pub struct NSBrowser;

    #[cfg(feature = "AppKit_NSBrowser")]
    unsafe impl ClassType for NSBrowser {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSAccessibility for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSAccessibilityElementProtocol for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSAnimatablePropertyContainer for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSAppearanceCustomization for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSCoding for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSDraggingDestination for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSObjectProtocol for NSBrowser {}

#[cfg(feature = "AppKit_NSBrowser")]
unsafe impl NSUserInterfaceItemIdentification for NSBrowser {}

extern_methods!(
    #[cfg(feature = "AppKit_NSBrowser")]
    unsafe impl NSBrowser {
        #[method(cellClass)]
        pub unsafe fn cellClass() -> &'static AnyClass;

        #[method(loadColumnZero)]
        pub unsafe fn loadColumnZero(&self);

        #[method(isLoaded)]
        pub unsafe fn isLoaded(&self) -> bool;

        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, double_action: Option<Sel>);

        #[method(setCellClass:)]
        pub unsafe fn setCellClass(&self, factory_id: &AnyClass);

        #[method_id(@__retain_semantics Other cellPrototype)]
        pub unsafe fn cellPrototype(&self) -> Option<Id<AnyObject>>;

        #[method(setCellPrototype:)]
        pub unsafe fn setCellPrototype(&self, cell_prototype: Option<&AnyObject>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSBrowserDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSBrowserDelegate>>);

        #[method(reusesColumns)]
        pub unsafe fn reusesColumns(&self) -> bool;

        #[method(setReusesColumns:)]
        pub unsafe fn setReusesColumns(&self, reuses_columns: bool);

        #[method(hasHorizontalScroller)]
        pub unsafe fn hasHorizontalScroller(&self) -> bool;

        #[method(setHasHorizontalScroller:)]
        pub unsafe fn setHasHorizontalScroller(&self, has_horizontal_scroller: bool);

        #[method(autohidesScroller)]
        pub unsafe fn autohidesScroller(&self) -> bool;

        #[method(setAutohidesScroller:)]
        pub unsafe fn setAutohidesScroller(&self, autohides_scroller: bool);

        #[method(separatesColumns)]
        pub unsafe fn separatesColumns(&self) -> bool;

        #[method(setSeparatesColumns:)]
        pub unsafe fn setSeparatesColumns(&self, separates_columns: bool);

        #[method(isTitled)]
        pub unsafe fn isTitled(&self) -> bool;

        #[method(setTitled:)]
        pub unsafe fn setTitled(&self, titled: bool);

        #[method(minColumnWidth)]
        pub unsafe fn minColumnWidth(&self) -> CGFloat;

        #[method(setMinColumnWidth:)]
        pub unsafe fn setMinColumnWidth(&self, min_column_width: CGFloat);

        #[method(maxVisibleColumns)]
        pub unsafe fn maxVisibleColumns(&self) -> NSInteger;

        #[method(setMaxVisibleColumns:)]
        pub unsafe fn setMaxVisibleColumns(&self, max_visible_columns: NSInteger);

        #[method(allowsMultipleSelection)]
        pub unsafe fn allowsMultipleSelection(&self) -> bool;

        #[method(setAllowsMultipleSelection:)]
        pub unsafe fn setAllowsMultipleSelection(&self, allows_multiple_selection: bool);

        #[method(allowsBranchSelection)]
        pub unsafe fn allowsBranchSelection(&self) -> bool;

        #[method(setAllowsBranchSelection:)]
        pub unsafe fn setAllowsBranchSelection(&self, allows_branch_selection: bool);

        #[method(allowsEmptySelection)]
        pub unsafe fn allowsEmptySelection(&self) -> bool;

        #[method(setAllowsEmptySelection:)]
        pub unsafe fn setAllowsEmptySelection(&self, allows_empty_selection: bool);

        #[method(takesTitleFromPreviousColumn)]
        pub unsafe fn takesTitleFromPreviousColumn(&self) -> bool;

        #[method(setTakesTitleFromPreviousColumn:)]
        pub unsafe fn setTakesTitleFromPreviousColumn(
            &self,
            takes_title_from_previous_column: bool,
        );

        #[method(sendsActionOnArrowKeys)]
        pub unsafe fn sendsActionOnArrowKeys(&self) -> bool;

        #[method(setSendsActionOnArrowKeys:)]
        pub unsafe fn setSendsActionOnArrowKeys(&self, sends_action_on_arrow_keys: bool);

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other itemAtIndexPath:)]
        pub unsafe fn itemAtIndexPath(&self, index_path: &NSIndexPath) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other itemAtRow:inColumn:)]
        pub unsafe fn itemAtRow_inColumn(
            &self,
            row: NSInteger,
            column: NSInteger,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other indexPathForColumn:)]
        pub unsafe fn indexPathForColumn(&self, column: NSInteger) -> Id<NSIndexPath>;

        #[method(isLeafItem:)]
        pub unsafe fn isLeafItem(&self, item: Option<&AnyObject>) -> bool;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(reloadDataForRowIndexes:inColumn:)]
        pub unsafe fn reloadDataForRowIndexes_inColumn(
            &self,
            row_indexes: &NSIndexSet,
            column: NSInteger,
        );

        #[method_id(@__retain_semantics Other parentForItemsInColumn:)]
        pub unsafe fn parentForItemsInColumn(&self, column: NSInteger) -> Option<Id<AnyObject>>;

        #[method(scrollRowToVisible:inColumn:)]
        pub unsafe fn scrollRowToVisible_inColumn(&self, row: NSInteger, column: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:ofColumn:)]
        pub unsafe fn setTitle_ofColumn(&self, string: &NSString, column: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other titleOfColumn:)]
        pub unsafe fn titleOfColumn(&self, column: NSInteger) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathSeparator)]
        pub unsafe fn pathSeparator(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPathSeparator:)]
        pub unsafe fn setPathSeparator(&self, path_separator: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPath:)]
        pub unsafe fn setPath(&self, path: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other path)]
        pub unsafe fn path(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathToColumn:)]
        pub unsafe fn pathToColumn(&self, column: NSInteger) -> Id<NSString>;

        #[method(clickedColumn)]
        pub unsafe fn clickedColumn(&self) -> NSInteger;

        #[method(clickedRow)]
        pub unsafe fn clickedRow(&self) -> NSInteger;

        #[method(selectedColumn)]
        pub unsafe fn selectedColumn(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other selectedCell)]
        pub unsafe fn selectedCell(&self) -> Option<Id<AnyObject>>;

        #[method_id(@__retain_semantics Other selectedCellInColumn:)]
        pub unsafe fn selectedCellInColumn(&self, column: NSInteger) -> Option<Id<AnyObject>>;

        #[cfg(all(feature = "AppKit_NSCell", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other selectedCells)]
        pub unsafe fn selectedCells(&self) -> Option<Id<NSArray<NSCell>>>;

        #[method(selectRow:inColumn:)]
        pub unsafe fn selectRow_inColumn(&self, row: NSInteger, column: NSInteger);

        #[method(selectedRowInColumn:)]
        pub unsafe fn selectedRowInColumn(&self, column: NSInteger) -> NSInteger;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method_id(@__retain_semantics Other selectionIndexPath)]
        pub unsafe fn selectionIndexPath(&self) -> Option<Id<NSIndexPath>>;

        #[cfg(feature = "Foundation_NSIndexPath")]
        #[method(setSelectionIndexPath:)]
        pub unsafe fn setSelectionIndexPath(&self, selection_index_path: Option<&NSIndexPath>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexPath"))]
        #[method_id(@__retain_semantics Other selectionIndexPaths)]
        pub unsafe fn selectionIndexPaths(&self) -> Id<NSArray<NSIndexPath>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexPath"))]
        #[method(setSelectionIndexPaths:)]
        pub unsafe fn setSelectionIndexPaths(&self, selection_index_paths: &NSArray<NSIndexPath>);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(selectRowIndexes:inColumn:)]
        pub unsafe fn selectRowIndexes_inColumn(&self, indexes: &NSIndexSet, column: NSInteger);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other selectedRowIndexesInColumn:)]
        pub unsafe fn selectedRowIndexesInColumn(
            &self,
            column: NSInteger,
        ) -> Option<Id<NSIndexSet>>;

        #[method(reloadColumn:)]
        pub unsafe fn reloadColumn(&self, column: NSInteger);

        #[method(validateVisibleColumns)]
        pub unsafe fn validateVisibleColumns(&self);

        #[method(scrollColumnsRightBy:)]
        pub unsafe fn scrollColumnsRightBy(&self, shift_amount: NSInteger);

        #[method(scrollColumnsLeftBy:)]
        pub unsafe fn scrollColumnsLeftBy(&self, shift_amount: NSInteger);

        #[method(scrollColumnToVisible:)]
        pub unsafe fn scrollColumnToVisible(&self, column: NSInteger);

        #[method(lastColumn)]
        pub unsafe fn lastColumn(&self) -> NSInteger;

        #[method(setLastColumn:)]
        pub unsafe fn setLastColumn(&self, last_column: NSInteger);

        #[method(addColumn)]
        pub unsafe fn addColumn(&self);

        #[method(numberOfVisibleColumns)]
        pub unsafe fn numberOfVisibleColumns(&self) -> NSInteger;

        #[method(firstVisibleColumn)]
        pub unsafe fn firstVisibleColumn(&self) -> NSInteger;

        #[method(lastVisibleColumn)]
        pub unsafe fn lastVisibleColumn(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other loadedCellAtRow:column:)]
        pub unsafe fn loadedCellAtRow_column(
            &self,
            row: NSInteger,
            col: NSInteger,
        ) -> Option<Id<AnyObject>>;

        #[method(selectAll:)]
        pub unsafe fn selectAll(&self, sender: Option<&AnyObject>);

        #[method(tile)]
        pub unsafe fn tile(&self);

        #[method(doClick:)]
        pub unsafe fn doClick(&self, sender: Option<&AnyObject>);

        #[method(doDoubleClick:)]
        pub unsafe fn doDoubleClick(&self, sender: Option<&AnyObject>);

        #[method(sendAction)]
        pub unsafe fn sendAction(&self) -> bool;

        #[method(titleFrameOfColumn:)]
        pub unsafe fn titleFrameOfColumn(&self, column: NSInteger) -> NSRect;

        #[method(drawTitleOfColumn:inRect:)]
        pub unsafe fn drawTitleOfColumn_inRect(&self, column: NSInteger, rect: NSRect);

        #[method(titleHeight)]
        pub unsafe fn titleHeight(&self) -> CGFloat;

        #[method(frameOfColumn:)]
        pub unsafe fn frameOfColumn(&self, column: NSInteger) -> NSRect;

        #[method(frameOfInsideOfColumn:)]
        pub unsafe fn frameOfInsideOfColumn(&self, column: NSInteger) -> NSRect;

        #[method(frameOfRow:inColumn:)]
        pub unsafe fn frameOfRow_inColumn(&self, row: NSInteger, column: NSInteger) -> NSRect;

        #[method(getRow:column:forPoint:)]
        pub unsafe fn getRow_column_forPoint(
            &self,
            row: *mut NSInteger,
            column: *mut NSInteger,
            point: NSPoint,
        ) -> bool;

        #[method(columnWidthForColumnContentWidth:)]
        pub unsafe fn columnWidthForColumnContentWidth(
            &self,
            column_content_width: CGFloat,
        ) -> CGFloat;

        #[method(columnContentWidthForColumnWidth:)]
        pub unsafe fn columnContentWidthForColumnWidth(&self, column_width: CGFloat) -> CGFloat;

        #[method(columnResizingType)]
        pub unsafe fn columnResizingType(&self) -> NSBrowserColumnResizingType;

        #[method(setColumnResizingType:)]
        pub unsafe fn setColumnResizingType(
            &self,
            column_resizing_type: NSBrowserColumnResizingType,
        );

        #[method(prefersAllColumnUserResizing)]
        pub unsafe fn prefersAllColumnUserResizing(&self) -> bool;

        #[method(setPrefersAllColumnUserResizing:)]
        pub unsafe fn setPrefersAllColumnUserResizing(
            &self,
            prefers_all_column_user_resizing: bool,
        );

        #[method(setWidth:ofColumn:)]
        pub unsafe fn setWidth_ofColumn(&self, column_width: CGFloat, column_index: NSInteger);

        #[method(widthOfColumn:)]
        pub unsafe fn widthOfColumn(&self, column: NSInteger) -> CGFloat;

        #[method(rowHeight)]
        pub unsafe fn rowHeight(&self) -> CGFloat;

        #[method(setRowHeight:)]
        pub unsafe fn setRowHeight(&self, row_height: CGFloat);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(noteHeightOfRowsWithIndexesChanged:inColumn:)]
        pub unsafe fn noteHeightOfRowsWithIndexesChanged_inColumn(
            &self,
            index_set: &NSIndexSet,
            column_index: NSInteger,
        );

        #[method(setDefaultColumnWidth:)]
        pub unsafe fn setDefaultColumnWidth(&self, column_width: CGFloat);

        #[method(defaultColumnWidth)]
        pub unsafe fn defaultColumnWidth(&self) -> CGFloat;

        #[method_id(@__retain_semantics Other columnsAutosaveName)]
        pub unsafe fn columnsAutosaveName(&self) -> Id<NSBrowserColumnsAutosaveName>;

        #[method(setColumnsAutosaveName:)]
        pub unsafe fn setColumnsAutosaveName(
            &self,
            columns_autosave_name: &NSBrowserColumnsAutosaveName,
        );

        #[method(removeSavedColumnsWithAutosaveName:)]
        pub unsafe fn removeSavedColumnsWithAutosaveName(name: &NSBrowserColumnsAutosaveName);

        #[cfg(all(feature = "AppKit_NSEvent", feature = "Foundation_NSIndexSet"))]
        #[method(canDragRowsWithIndexes:inColumn:withEvent:)]
        pub unsafe fn canDragRowsWithIndexes_inColumn_withEvent(
            &self,
            row_indexes: &NSIndexSet,
            column: NSInteger,
            event: &NSEvent,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSEvent",
            feature = "AppKit_NSImage",
            feature = "Foundation_NSIndexSet"
        ))]
        #[method_id(@__retain_semantics Other draggingImageForRowsWithIndexes:inColumn:withEvent:offset:)]
        pub unsafe fn draggingImageForRowsWithIndexes_inColumn_withEvent_offset(
            &self,
            row_indexes: &NSIndexSet,
            column: NSInteger,
            event: &NSEvent,
            drag_image_offset: NSPointPointer,
        ) -> Option<Id<NSImage>>;

        #[method(setDraggingSourceOperationMask:forLocal:)]
        pub unsafe fn setDraggingSourceOperationMask_forLocal(
            &self,
            mask: NSDragOperation,
            is_local: bool,
        );

        #[method(allowsTypeSelect)]
        pub unsafe fn allowsTypeSelect(&self) -> bool;

        #[method(setAllowsTypeSelect:)]
        pub unsafe fn setAllowsTypeSelect(&self, allows_type_select: bool);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[cfg(all(feature = "AppKit_NSEvent", feature = "Foundation_NSIndexPath"))]
        #[method(editItemAtIndexPath:withEvent:select:)]
        pub unsafe fn editItemAtIndexPath_withEvent_select(
            &self,
            index_path: &NSIndexPath,
            event: Option<&NSEvent>,
            select: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSBrowser")]
    unsafe impl NSBrowser {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

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
    #[cfg(feature = "AppKit_NSBrowser")]
    unsafe impl NSBrowser {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSBrowser")]
    unsafe impl NSBrowser {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSBrowserColumnConfigurationDidChangeNotification: &'static NSNotificationName);

extern_protocol!(
    pub unsafe trait NSBrowserDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:numberOfRowsInColumn:)]
        unsafe fn browser_numberOfRowsInColumn(
            &self,
            sender: &NSBrowser,
            column: NSInteger,
        ) -> NSInteger;

        #[cfg(all(feature = "AppKit_NSBrowser", feature = "AppKit_NSMatrix"))]
        #[optional]
        #[method(browser:createRowsForColumn:inMatrix:)]
        unsafe fn browser_createRowsForColumn_inMatrix(
            &self,
            sender: &NSBrowser,
            column: NSInteger,
            matrix: &NSMatrix,
        );

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:numberOfChildrenOfItem:)]
        unsafe fn browser_numberOfChildrenOfItem(
            &self,
            browser: &NSBrowser,
            item: Option<&AnyObject>,
        ) -> NSInteger;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method_id(@__retain_semantics Other browser:child:ofItem:)]
        unsafe fn browser_child_ofItem(
            &self,
            browser: &NSBrowser,
            index: NSInteger,
            item: Option<&AnyObject>,
        ) -> Id<AnyObject>;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:isLeafItem:)]
        unsafe fn browser_isLeafItem(&self, browser: &NSBrowser, item: Option<&AnyObject>) -> bool;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method_id(@__retain_semantics Other browser:objectValueForItem:)]
        unsafe fn browser_objectValueForItem(
            &self,
            browser: &NSBrowser,
            item: Option<&AnyObject>,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:heightOfRow:inColumn:)]
        unsafe fn browser_heightOfRow_inColumn(
            &self,
            browser: &NSBrowser,
            row: NSInteger,
            column_index: NSInteger,
        ) -> CGFloat;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method_id(@__retain_semantics Other rootItemForBrowser:)]
        unsafe fn rootItemForBrowser(&self, browser: &NSBrowser) -> Option<Id<AnyObject>>;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:setObjectValue:forItem:)]
        unsafe fn browser_setObjectValue_forItem(
            &self,
            browser: &NSBrowser,
            object: Option<&AnyObject>,
            item: Option<&AnyObject>,
        );

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:shouldEditItem:)]
        unsafe fn browser_shouldEditItem(
            &self,
            browser: &NSBrowser,
            item: Option<&AnyObject>,
        ) -> bool;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:willDisplayCell:atRow:column:)]
        unsafe fn browser_willDisplayCell_atRow_column(
            &self,
            sender: &NSBrowser,
            cell: &AnyObject,
            row: NSInteger,
            column: NSInteger,
        );

        #[cfg(all(feature = "AppKit_NSBrowser", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other browser:titleOfColumn:)]
        unsafe fn browser_titleOfColumn(
            &self,
            sender: &NSBrowser,
            column: NSInteger,
        ) -> Option<Id<NSString>>;

        #[cfg(all(feature = "AppKit_NSBrowser", feature = "Foundation_NSString"))]
        #[optional]
        #[method(browser:selectCellWithString:inColumn:)]
        unsafe fn browser_selectCellWithString_inColumn(
            &self,
            sender: &NSBrowser,
            title: &NSString,
            column: NSInteger,
        ) -> bool;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:selectRow:inColumn:)]
        unsafe fn browser_selectRow_inColumn(
            &self,
            sender: &NSBrowser,
            row: NSInteger,
            column: NSInteger,
        ) -> bool;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:isColumnValid:)]
        unsafe fn browser_isColumnValid(&self, sender: &NSBrowser, column: NSInteger) -> bool;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browserWillScroll:)]
        unsafe fn browserWillScroll(&self, sender: &NSBrowser);

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browserDidScroll:)]
        unsafe fn browserDidScroll(&self, sender: &NSBrowser);

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:shouldSizeColumn:forUserResize:toWidth:)]
        unsafe fn browser_shouldSizeColumn_forUserResize_toWidth(
            &self,
            browser: &NSBrowser,
            column_index: NSInteger,
            for_user_resize: bool,
            suggested_width: CGFloat,
        ) -> CGFloat;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:sizeToFitWidthOfColumn:)]
        unsafe fn browser_sizeToFitWidthOfColumn(
            &self,
            browser: &NSBrowser,
            column_index: NSInteger,
        ) -> CGFloat;

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(browserColumnConfigurationDidChange:)]
        unsafe fn browserColumnConfigurationDidChange(&self, notification: &NSNotification);

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:shouldShowCellExpansionForRow:column:)]
        unsafe fn browser_shouldShowCellExpansionForRow_column(
            &self,
            browser: &NSBrowser,
            row: NSInteger,
            column: NSInteger,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSBrowser",
            feature = "AppKit_NSPasteboard",
            feature = "Foundation_NSIndexSet"
        ))]
        #[optional]
        #[method(browser:writeRowsWithIndexes:inColumn:toPasteboard:)]
        unsafe fn browser_writeRowsWithIndexes_inColumn_toPasteboard(
            &self,
            browser: &NSBrowser,
            row_indexes: &NSIndexSet,
            column: NSInteger,
            pasteboard: &NSPasteboard,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSBrowser",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSIndexSet",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[deprecated = "Use NSFilePromiseReceiver objects instead"]
        #[optional]
        #[method_id(@__retain_semantics Other browser:namesOfPromisedFilesDroppedAtDestination:forDraggedRowsWithIndexes:inColumn:)]
        unsafe fn browser_namesOfPromisedFilesDroppedAtDestination_forDraggedRowsWithIndexes_inColumn(
            &self,
            browser: &NSBrowser,
            drop_destination: &NSURL,
            row_indexes: &NSIndexSet,
            column: NSInteger,
        ) -> Id<NSArray<NSString>>;

        #[cfg(all(
            feature = "AppKit_NSBrowser",
            feature = "AppKit_NSEvent",
            feature = "Foundation_NSIndexSet"
        ))]
        #[optional]
        #[method(browser:canDragRowsWithIndexes:inColumn:withEvent:)]
        unsafe fn browser_canDragRowsWithIndexes_inColumn_withEvent(
            &self,
            browser: &NSBrowser,
            row_indexes: &NSIndexSet,
            column: NSInteger,
            event: &NSEvent,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSBrowser",
            feature = "AppKit_NSEvent",
            feature = "AppKit_NSImage",
            feature = "Foundation_NSIndexSet"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other browser:draggingImageForRowsWithIndexes:inColumn:withEvent:offset:)]
        unsafe fn browser_draggingImageForRowsWithIndexes_inColumn_withEvent_offset(
            &self,
            browser: &NSBrowser,
            row_indexes: &NSIndexSet,
            column: NSInteger,
            event: &NSEvent,
            drag_image_offset: NSPointPointer,
        ) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:validateDrop:proposedRow:column:dropOperation:)]
        unsafe fn browser_validateDrop_proposedRow_column_dropOperation(
            &self,
            browser: &NSBrowser,
            info: &ProtocolObject<dyn NSDraggingInfo>,
            row: NonNull<NSInteger>,
            column: NonNull<NSInteger>,
            drop_operation: NonNull<NSBrowserDropOperation>,
        ) -> NSDragOperation;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:acceptDrop:atRow:column:dropOperation:)]
        unsafe fn browser_acceptDrop_atRow_column_dropOperation(
            &self,
            browser: &NSBrowser,
            info: &ProtocolObject<dyn NSDraggingInfo>,
            row: NSInteger,
            column: NSInteger,
            drop_operation: NSBrowserDropOperation,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSBrowser", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other browser:typeSelectStringForRow:inColumn:)]
        unsafe fn browser_typeSelectStringForRow_inColumn(
            &self,
            browser: &NSBrowser,
            row: NSInteger,
            column: NSInteger,
        ) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "AppKit_NSBrowser",
            feature = "AppKit_NSEvent",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method(browser:shouldTypeSelectForEvent:withCurrentSearchString:)]
        unsafe fn browser_shouldTypeSelectForEvent_withCurrentSearchString(
            &self,
            browser: &NSBrowser,
            event: &NSEvent,
            search_string: Option<&NSString>,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSBrowser", feature = "Foundation_NSString"))]
        #[optional]
        #[method(browser:nextTypeSelectMatchFromRow:toRow:inColumn:forString:)]
        unsafe fn browser_nextTypeSelectMatchFromRow_toRow_inColumn_forString(
            &self,
            browser: &NSBrowser,
            start_row: NSInteger,
            end_row: NSInteger,
            column: NSInteger,
            search_string: Option<&NSString>,
        ) -> NSInteger;

        #[cfg(all(feature = "AppKit_NSBrowser", feature = "AppKit_NSViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other browser:previewViewControllerForLeafItem:)]
        unsafe fn browser_previewViewControllerForLeafItem(
            &self,
            browser: &NSBrowser,
            item: &AnyObject,
        ) -> Option<Id<NSViewController>>;

        #[cfg(all(feature = "AppKit_NSBrowser", feature = "AppKit_NSViewController"))]
        #[optional]
        #[method_id(@__retain_semantics Other browser:headerViewControllerForItem:)]
        unsafe fn browser_headerViewControllerForItem(
            &self,
            browser: &NSBrowser,
            item: Option<&AnyObject>,
        ) -> Option<Id<NSViewController>>;

        #[cfg(feature = "AppKit_NSBrowser")]
        #[optional]
        #[method(browser:didChangeLastColumn:toColumn:)]
        unsafe fn browser_didChangeLastColumn_toColumn(
            &self,
            browser: &NSBrowser,
            old_last_column: NSInteger,
            column: NSInteger,
        );

        #[cfg(all(feature = "AppKit_NSBrowser", feature = "Foundation_NSIndexSet"))]
        #[optional]
        #[method_id(@__retain_semantics Other browser:selectionIndexesForProposedSelection:inColumn:)]
        unsafe fn browser_selectionIndexesForProposedSelection_inColumn(
            &self,
            browser: &NSBrowser,
            proposed_selection_indexes: &NSIndexSet,
            column: NSInteger,
        ) -> Id<NSIndexSet>;
    }

    unsafe impl ProtocolType for dyn NSBrowserDelegate {}
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSBrowser")]
    unsafe impl NSBrowser {
        #[deprecated]
        #[method(setAcceptsArrowKeys:)]
        pub unsafe fn setAcceptsArrowKeys(&self, flag: bool);

        #[deprecated]
        #[method(acceptsArrowKeys)]
        pub unsafe fn acceptsArrowKeys(&self) -> bool;

        #[deprecated]
        #[method(displayColumn:)]
        pub unsafe fn displayColumn(&self, column: NSInteger);

        #[deprecated]
        #[method(displayAllColumns)]
        pub unsafe fn displayAllColumns(&self);

        #[cfg(feature = "AppKit_NSScroller")]
        #[deprecated]
        #[method(scrollViaScroller:)]
        pub unsafe fn scrollViaScroller(&self, sender: Option<&NSScroller>);

        #[deprecated]
        #[method(updateScroller)]
        pub unsafe fn updateScroller(&self);

        #[deprecated = "Use the item based NSBrowser instead"]
        #[method(setMatrixClass:)]
        pub unsafe fn setMatrixClass(&self, factory_id: &AnyClass);

        #[deprecated = "Use the item based NSBrowser instead"]
        #[method(matrixClass)]
        pub unsafe fn matrixClass(&self) -> &'static AnyClass;

        #[cfg(feature = "AppKit_NSMatrix")]
        #[deprecated = "Use the item based NSBrowser instead"]
        #[method(columnOfMatrix:)]
        pub unsafe fn columnOfMatrix(&self, matrix: &NSMatrix) -> NSInteger;

        #[cfg(feature = "AppKit_NSMatrix")]
        #[deprecated = "Use the item based NSBrowser instead"]
        #[method_id(@__retain_semantics Other matrixInColumn:)]
        pub unsafe fn matrixInColumn(&self, column: NSInteger) -> Option<Id<NSMatrix>>;
    }
);
