/// All available actions in the TUI
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    // Navigation
    Quit,
    ToggleFocus,
    Refresh,

    // Document operations - Phase 1
    CreateDocument,
    UpdateDocument,
    DeleteDocument,
    SearchDocuments,

    // Document operations - Phase 2
    ArchiveDocument,
    UnarchiveDocument,
    StarDocument,
    UnstarDocument,
    UnpublishDocument,
    TemplatizeDocument,
    MoveDocument,
    RestoreDocument,
    ExportDocument,
    ViewDrafts,
    ViewTemplates,
    ViewRecent,

    // Collection operations - Phase 1
    CreateCollection,
    UpdateCollection,
    DeleteCollection,

    // Collection operations - Phase 2
    ExportCollection,
    ExportAllCollections,
    ImportFileToCollection,
    MoveCollection,
    AddUserToCollection,
    RemoveUserFromCollection,
    AddGroupToCollection,
    RemoveGroupFromCollection,
    ViewCollectionMemberships,

    // User operations - Phase 3
    ViewCurrentUser,
    ViewUser,
    ListUsers,
    UpdateUser,
    SuspendUser,
    ActivateUser,
    DeleteUser,
    PromoteUser,
    DemoteUser,
    InviteUser,

    // Comment operations - Phase 3
    CreateComment,
    ViewComments,
    UpdateComment,
    DeleteComment,
    ResolveComment,
    UnresolveComment,

    // Group operations - Phase 3
    CreateGroup,
    ListGroups,
    ViewGroup,
    UpdateGroup,
    DeleteGroup,
    AddUserToGroup,
    RemoveUserFromGroup,
    ViewGroupMemberships,

    // Share operations - Phase 3
    CreateShare,
    ListShares,
    ViewShare,
    UpdateShare,
    RevokeShare,

    // Attachment operations - Phase 3
    UploadAttachment,
    ListAttachments,
    DeleteAttachment,
    GetAttachmentUrl,

    // UI actions
    ShowHelp,
    ShowMenu,
    EnterCommandMode,
}

impl Action {
    pub fn description(&self) -> &str {
        match self {
            // Navigation
            Action::Quit => "Quit application",
            Action::ToggleFocus => "Switch between sidebar and editor",
            Action::Refresh => "Refresh data from server",

            // Document operations - Phase 1
            Action::CreateDocument => "Create new document",
            Action::UpdateDocument => "Update current document",
            Action::DeleteDocument => "Delete current document",
            Action::SearchDocuments => "Search documents",

            // Document operations - Phase 2
            Action::ArchiveDocument => "Archive current document",
            Action::UnarchiveDocument => "Unarchive current document",
            Action::StarDocument => "Star current document",
            Action::UnstarDocument => "Unstar current document",
            Action::UnpublishDocument => "Unpublish current document",
            Action::TemplatizeDocument => "Convert to template",
            Action::MoveDocument => "Move document to different location",
            Action::RestoreDocument => "Restore document from trash",
            Action::ExportDocument => "Export current document",
            Action::ViewDrafts => "View draft documents",
            Action::ViewTemplates => "View template documents",
            Action::ViewRecent => "View recently viewed documents",

            // Collection operations - Phase 1
            Action::CreateCollection => "Create new collection",
            Action::UpdateCollection => "Update collection",
            Action::DeleteCollection => "Delete collection",

            // Collection operations - Phase 2
            Action::ExportCollection => "Export collection",
            Action::ExportAllCollections => "Export all collections",
            Action::ImportFileToCollection => "Import file into collection",
            Action::MoveCollection => "Move collection",
            Action::AddUserToCollection => "Add user to collection",
            Action::RemoveUserFromCollection => "Remove user from collection",
            Action::AddGroupToCollection => "Add group to collection",
            Action::RemoveGroupFromCollection => "Remove group from collection",
            Action::ViewCollectionMemberships => "View collection members",

            // User operations - Phase 3
            Action::ViewCurrentUser => "View your profile",
            Action::ViewUser => "View user profile",
            Action::ListUsers => "List all users",
            Action::UpdateUser => "Update user profile",
            Action::SuspendUser => "Suspend user account",
            Action::ActivateUser => "Activate user account",
            Action::DeleteUser => "Delete user account",
            Action::PromoteUser => "Promote user to admin",
            Action::DemoteUser => "Demote user from admin",
            Action::InviteUser => "Invite new user",

            // Comment operations - Phase 3
            Action::CreateComment => "Add comment to document",
            Action::ViewComments => "View document comments",
            Action::UpdateComment => "Update comment",
            Action::DeleteComment => "Delete comment",
            Action::ResolveComment => "Resolve comment thread",
            Action::UnresolveComment => "Unresolve comment thread",

            // Group operations - Phase 3
            Action::CreateGroup => "Create new group",
            Action::ListGroups => "List all groups",
            Action::ViewGroup => "View group details",
            Action::UpdateGroup => "Update group",
            Action::DeleteGroup => "Delete group",
            Action::AddUserToGroup => "Add user to group",
            Action::RemoveUserFromGroup => "Remove user from group",
            Action::ViewGroupMemberships => "View group members",

            // Share operations - Phase 3
            Action::CreateShare => "Create public share link",
            Action::ListShares => "List all shares",
            Action::ViewShare => "View share details",
            Action::UpdateShare => "Update share settings",
            Action::RevokeShare => "Revoke share link",

            // Attachment operations - Phase 3
            Action::UploadAttachment => "Upload file attachment",
            Action::ListAttachments => "List all attachments",
            Action::DeleteAttachment => "Delete attachment",
            Action::GetAttachmentUrl => "Get attachment download URL",

            // UI actions
            Action::ShowHelp => "Show help screen",
            Action::ShowMenu => "Show action menu",
            Action::EnterCommandMode => "Enter command mode",
        }
    }

    pub fn category(&self) -> &str {
        match self {
            Action::Quit | Action::ToggleFocus | Action::Refresh | Action::ShowHelp | Action::ShowMenu | Action::EnterCommandMode => "Navigation",
            Action::CreateDocument | Action::UpdateDocument | Action::DeleteDocument | Action::SearchDocuments |
            Action::ArchiveDocument | Action::UnarchiveDocument | Action::StarDocument | Action::UnstarDocument |
            Action::UnpublishDocument | Action::TemplatizeDocument | Action::MoveDocument | Action::RestoreDocument |
            Action::ExportDocument | Action::ViewDrafts | Action::ViewTemplates | Action::ViewRecent => "Documents",
            Action::CreateCollection | Action::UpdateCollection | Action::DeleteCollection | Action::ExportCollection |
            Action::ExportAllCollections | Action::ImportFileToCollection | Action::MoveCollection |
            Action::AddUserToCollection | Action::RemoveUserFromCollection | Action::AddGroupToCollection |
            Action::RemoveGroupFromCollection | Action::ViewCollectionMemberships => "Collections",
            Action::ViewCurrentUser | Action::ViewUser | Action::ListUsers | Action::UpdateUser |
            Action::SuspendUser | Action::ActivateUser | Action::DeleteUser | Action::PromoteUser |
            Action::DemoteUser | Action::InviteUser => "Users",
            Action::CreateComment | Action::ViewComments | Action::UpdateComment | Action::DeleteComment |
            Action::ResolveComment | Action::UnresolveComment => "Comments",
            Action::CreateGroup | Action::ListGroups | Action::ViewGroup | Action::UpdateGroup |
            Action::DeleteGroup | Action::AddUserToGroup | Action::RemoveUserFromGroup |
            Action::ViewGroupMemberships => "Groups",
            Action::CreateShare | Action::ListShares | Action::ViewShare | Action::UpdateShare |
            Action::RevokeShare => "Shares",
            Action::UploadAttachment | Action::ListAttachments | Action::DeleteAttachment |
            Action::GetAttachmentUrl => "Attachments",
        }
    }
}
