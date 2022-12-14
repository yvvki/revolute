# Rust API client for revolt_api

Open source user-first chat platform.

For more information, please visit [https://revolt.chat](https://revolt.chat)

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.5.5
- Package version: 0.5.5
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `revolt_api` and add the following to `Cargo.toml` under `[dependencies]`:

```
revolt_api = { path = "./revolt_api" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.revolt.chat*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountApi* | [**change_email_change_email**](docs/AccountApi.md#change_email_change_email) | **PATCH** /auth/account/change/email | Change Email
*AccountApi* | [**change_password_change_password**](docs/AccountApi.md#change_password_change_password) | **PATCH** /auth/account/change/password | Change Password
*AccountApi* | [**confirm_deletion_confirm_deletion**](docs/AccountApi.md#confirm_deletion_confirm_deletion) | **PUT** /auth/account/delete | Confirm Account Deletion
*AccountApi* | [**create_account_create_account**](docs/AccountApi.md#create_account_create_account) | **POST** /auth/account/create | Create Account
*AccountApi* | [**delete_account_delete_account**](docs/AccountApi.md#delete_account_delete_account) | **POST** /auth/account/delete | Delete Account
*AccountApi* | [**disable_account_disable_account**](docs/AccountApi.md#disable_account_disable_account) | **POST** /auth/account/disable | Disable Account
*AccountApi* | [**fetch_account_fetch_account**](docs/AccountApi.md#fetch_account_fetch_account) | **GET** /auth/account/ | Fetch Account
*AccountApi* | [**password_reset_password_reset**](docs/AccountApi.md#password_reset_password_reset) | **PATCH** /auth/account/reset_password | Password Reset
*AccountApi* | [**resend_verification_resend_verification**](docs/AccountApi.md#resend_verification_resend_verification) | **POST** /auth/account/reverify | Resend Verification
*AccountApi* | [**send_password_reset_send_password_reset**](docs/AccountApi.md#send_password_reset_send_password_reset) | **POST** /auth/account/reset_password | Send Password Reset
*AccountApi* | [**verify_email_verify_email**](docs/AccountApi.md#verify_email_verify_email) | **POST** /auth/account/verify/{code} | Verify Email
*BotsApi* | [**create_create_bot**](docs/BotsApi.md#create_create_bot) | **POST** /bots/create | Create Bot
*BotsApi* | [**delete_delete_bot**](docs/BotsApi.md#delete_delete_bot) | **DELETE** /bots/{target} | Delete Bot
*BotsApi* | [**edit_edit_bot**](docs/BotsApi.md#edit_edit_bot) | **PATCH** /bots/{target} | Edit Bot
*BotsApi* | [**fetch_fetch_bot**](docs/BotsApi.md#fetch_fetch_bot) | **GET** /bots/{target} | Fetch Bot
*BotsApi* | [**fetch_owned_fetch_owned_bots**](docs/BotsApi.md#fetch_owned_fetch_owned_bots) | **GET** /bots/@me | Fetch Owned Bots
*BotsApi* | [**fetch_public_fetch_public_bot**](docs/BotsApi.md#fetch_public_fetch_public_bot) | **GET** /bots/{target}/invite | Fetch Public Bot
*BotsApi* | [**invite_invite_bot**](docs/BotsApi.md#invite_invite_bot) | **POST** /bots/{target}/invite | Invite Bot
*ChannelInformationApi* | [**channel_delete_req**](docs/ChannelInformationApi.md#channel_delete_req) | **DELETE** /channels/{target} | Close Channel
*ChannelInformationApi* | [**channel_edit_req**](docs/ChannelInformationApi.md#channel_edit_req) | **PATCH** /channels/{target} | Edit Channel
*ChannelInformationApi* | [**channel_fetch_req**](docs/ChannelInformationApi.md#channel_fetch_req) | **GET** /channels/{target} | Fetch Channel
*ChannelInvitesApi* | [**invite_create_req**](docs/ChannelInvitesApi.md#invite_create_req) | **POST** /channels/{target}/invites | Create Invite
*ChannelPermissionsApi* | [**permissions_set_default_req**](docs/ChannelPermissionsApi.md#permissions_set_default_req) | **PUT** /channels/{target}/permissions/default | Set Default Permission
*ChannelPermissionsApi* | [**permissions_set_req**](docs/ChannelPermissionsApi.md#permissions_set_req) | **PUT** /channels/{target}/permissions/{role_id} | Set Role Permission
*CoreApi* | [**root_root**](docs/CoreApi.md#root_root) | **GET** / | Query Node
*DirectMessagingApi* | [**fetch_dms_req**](docs/DirectMessagingApi.md#fetch_dms_req) | **GET** /users/dms | Fetch Direct Message Channels
*DirectMessagingApi* | [**open_dm_req**](docs/DirectMessagingApi.md#open_dm_req) | **GET** /users/{target}/dm | Open Direct Message
*EmojisApi* | [**emoji_create_create_emoji**](docs/EmojisApi.md#emoji_create_create_emoji) | **PUT** /custom/emoji/{id} | Create New Emoji
*EmojisApi* | [**emoji_delete_delete_emoji**](docs/EmojisApi.md#emoji_delete_delete_emoji) | **DELETE** /custom/emoji/{id} | Delete Emoji
*EmojisApi* | [**emoji_fetch_fetch_emoji**](docs/EmojisApi.md#emoji_fetch_fetch_emoji) | **GET** /custom/emoji/{id} | Fetch Emoji
*GroupsApi* | [**group_add_member_req**](docs/GroupsApi.md#group_add_member_req) | **PUT** /channels/{target}/recipients/{member} | Add Member to Group
*GroupsApi* | [**group_create_req**](docs/GroupsApi.md#group_create_req) | **POST** /channels/create | Create Group
*GroupsApi* | [**group_remove_member_req**](docs/GroupsApi.md#group_remove_member_req) | **DELETE** /channels/{target}/recipients/{member} | Remove Member from Group
*GroupsApi* | [**members_fetch_req**](docs/GroupsApi.md#members_fetch_req) | **GET** /channels/{target}/members | Fetch Group Members
*InteractionsApi* | [**message_clear_reactions_clear_reactions**](docs/InteractionsApi.md#message_clear_reactions_clear_reactions) | **DELETE** /channels/{target}/messages/{msg}/reactions | Remove All Reactions from Message
*InteractionsApi* | [**message_react_react_message**](docs/InteractionsApi.md#message_react_react_message) | **PUT** /channels/{target}/messages/{msg}/reactions/{emoji} | Add Reaction to Message
*InteractionsApi* | [**message_unreact_unreact_message**](docs/InteractionsApi.md#message_unreact_unreact_message) | **DELETE** /channels/{target}/messages/{msg}/reactions/{emoji} | Remove Reaction(s) to Message
*InvitesApi* | [**invite_delete_req**](docs/InvitesApi.md#invite_delete_req) | **DELETE** /invites/{target} | Delete Invite
*InvitesApi* | [**invite_fetch_req**](docs/InvitesApi.md#invite_fetch_req) | **GET** /invites/{target} | Fetch Invite
*InvitesApi* | [**invite_join_req**](docs/InvitesApi.md#invite_join_req) | **POST** /invites/{target} | Join Invite
*MfaApi* | [**create_ticket_create_ticket**](docs/MfaApi.md#create_ticket_create_ticket) | **PUT** /auth/mfa/ticket | Create MFA ticket
*MfaApi* | [**fetch_recovery_fetch_recovery**](docs/MfaApi.md#fetch_recovery_fetch_recovery) | **POST** /auth/mfa/recovery | Fetch Recovery Codes
*MfaApi* | [**fetch_status_fetch_status**](docs/MfaApi.md#fetch_status_fetch_status) | **GET** /auth/mfa/ | MFA Status
*MfaApi* | [**generate_recovery_generate_recovery**](docs/MfaApi.md#generate_recovery_generate_recovery) | **PATCH** /auth/mfa/recovery | Generate Recovery Codes
*MfaApi* | [**get_mfa_methods_get_mfa_methods**](docs/MfaApi.md#get_mfa_methods_get_mfa_methods) | **GET** /auth/mfa/methods | Get MFA Methods
*MfaApi* | [**totp_disable_totp_disable**](docs/MfaApi.md#totp_disable_totp_disable) | **DELETE** /auth/mfa/totp | Disable TOTP 2FA
*MfaApi* | [**totp_enable_totp_enable**](docs/MfaApi.md#totp_enable_totp_enable) | **PUT** /auth/mfa/totp | Enable TOTP 2FA
*MfaApi* | [**totp_generate_secret_totp_generate_secret**](docs/MfaApi.md#totp_generate_secret_totp_generate_secret) | **POST** /auth/mfa/totp | Generate TOTP Secret
*MessagingApi* | [**channel_ack_req**](docs/MessagingApi.md#channel_ack_req) | **PUT** /channels/{target}/ack/{message} | Acknowledge Message
*MessagingApi* | [**message_bulk_delete_req**](docs/MessagingApi.md#message_bulk_delete_req) | **DELETE** /channels/{target}/messages/bulk | Bulk Delete Messages
*MessagingApi* | [**message_delete_req**](docs/MessagingApi.md#message_delete_req) | **DELETE** /channels/{target}/messages/{msg} | Delete Message
*MessagingApi* | [**message_edit_req**](docs/MessagingApi.md#message_edit_req) | **PATCH** /channels/{target}/messages/{msg} | Edit Message
*MessagingApi* | [**message_fetch_req**](docs/MessagingApi.md#message_fetch_req) | **GET** /channels/{target}/messages/{msg} | Fetch Message
*MessagingApi* | [**message_query_req**](docs/MessagingApi.md#message_query_req) | **GET** /channels/{target}/messages | Fetch Messages
*MessagingApi* | [**message_query_stale_req**](docs/MessagingApi.md#message_query_stale_req) | **POST** /channels/{_target}/messages/stale | Poll Message Changes
*MessagingApi* | [**message_search_req**](docs/MessagingApi.md#message_search_req) | **POST** /channels/{target}/search | Search for Messages
*MessagingApi* | [**message_send_message_send**](docs/MessagingApi.md#message_send_message_send) | **POST** /channels/{target}/messages | Send Message
*OnboardingApi* | [**complete_req**](docs/OnboardingApi.md#complete_req) | **POST** /onboard/complete | Complete Onboarding
*OnboardingApi* | [**hello_req**](docs/OnboardingApi.md#hello_req) | **GET** /onboard/hello | Check Onboarding Status
*RelationshipsApi* | [**add_friend_req**](docs/RelationshipsApi.md#add_friend_req) | **PUT** /users/{target}/friend | Accept Friend Request
*RelationshipsApi* | [**block_user_req**](docs/RelationshipsApi.md#block_user_req) | **PUT** /users/{target}/block | Block User
*RelationshipsApi* | [**find_mutual_req**](docs/RelationshipsApi.md#find_mutual_req) | **GET** /users/{target}/mutual | Fetch Mutual Friends And Servers
*RelationshipsApi* | [**remove_friend_req**](docs/RelationshipsApi.md#remove_friend_req) | **DELETE** /users/{target}/friend | Deny Friend Request / Remove Friend
*RelationshipsApi* | [**send_friend_request_req**](docs/RelationshipsApi.md#send_friend_request_req) | **POST** /users/friend | Send Friend Request
*RelationshipsApi* | [**unblock_user_req**](docs/RelationshipsApi.md#unblock_user_req) | **DELETE** /users/{target}/block | Unblock User
*ServerCustomisationApi* | [**emoji_list_list_emoji**](docs/ServerCustomisationApi.md#emoji_list_list_emoji) | **GET** /servers/{target}/emojis | Fetch Server Emoji
*ServerInformationApi* | [**channel_create_req**](docs/ServerInformationApi.md#channel_create_req) | **POST** /servers/{target}/channels | Create Channel
*ServerInformationApi* | [**server_ack_req**](docs/ServerInformationApi.md#server_ack_req) | **PUT** /servers/{target}/ack | Mark Server As Read
*ServerInformationApi* | [**server_create_req**](docs/ServerInformationApi.md#server_create_req) | **POST** /servers/create | Create Server
*ServerInformationApi* | [**server_delete_req**](docs/ServerInformationApi.md#server_delete_req) | **DELETE** /servers/{target} | Delete / Leave Server
*ServerInformationApi* | [**server_edit_req**](docs/ServerInformationApi.md#server_edit_req) | **PATCH** /servers/{target} | Edit Server
*ServerInformationApi* | [**server_fetch_req**](docs/ServerInformationApi.md#server_fetch_req) | **GET** /servers/{target} | Fetch Server
*ServerMembersApi* | [**ban_create_req**](docs/ServerMembersApi.md#ban_create_req) | **PUT** /servers/{server}/bans/{target} | Ban User
*ServerMembersApi* | [**ban_list_req**](docs/ServerMembersApi.md#ban_list_req) | **GET** /servers/{target}/bans | Fetch Bans
*ServerMembersApi* | [**ban_remove_req**](docs/ServerMembersApi.md#ban_remove_req) | **DELETE** /servers/{server}/bans/{target} | Unban user
*ServerMembersApi* | [**invites_fetch_req**](docs/ServerMembersApi.md#invites_fetch_req) | **GET** /servers/{target}/invites | Fetch Invites
*ServerMembersApi* | [**member_edit_req**](docs/ServerMembersApi.md#member_edit_req) | **PATCH** /servers/{server}/members/{target} | Edit Member
*ServerMembersApi* | [**member_fetch_all_req**](docs/ServerMembersApi.md#member_fetch_all_req) | **GET** /servers/{target}/members | Fetch Members
*ServerMembersApi* | [**member_fetch_req**](docs/ServerMembersApi.md#member_fetch_req) | **GET** /servers/{target}/members/{member} | Fetch Member
*ServerMembersApi* | [**member_remove_req**](docs/ServerMembersApi.md#member_remove_req) | **DELETE** /servers/{target}/members/{member} | Kick Member
*ServerPermissionsApi* | [**permissions_set_default_req**](docs/ServerPermissionsApi.md#permissions_set_default_req) | **PUT** /servers/{target}/permissions/default | Set Default Permission
*ServerPermissionsApi* | [**permissions_set_req**](docs/ServerPermissionsApi.md#permissions_set_req) | **PUT** /servers/{target}/permissions/{role_id} | Set Role Permission
*ServerPermissionsApi* | [**roles_create_req**](docs/ServerPermissionsApi.md#roles_create_req) | **POST** /servers/{target}/roles | Create Role
*ServerPermissionsApi* | [**roles_delete_req**](docs/ServerPermissionsApi.md#roles_delete_req) | **DELETE** /servers/{target}/roles/{role_id} | Delete Role
*ServerPermissionsApi* | [**roles_edit_req**](docs/ServerPermissionsApi.md#roles_edit_req) | **PATCH** /servers/{target}/roles/{role_id} | Edit Role
*SessionApi* | [**edit_edit**](docs/SessionApi.md#edit_edit) | **PATCH** /auth/session/{id} | Edit Session
*SessionApi* | [**fetch_all_fetch_all**](docs/SessionApi.md#fetch_all_fetch_all) | **GET** /auth/session/all | Fetch Sessions
*SessionApi* | [**login_login**](docs/SessionApi.md#login_login) | **POST** /auth/session/login | Login
*SessionApi* | [**logout_logout**](docs/SessionApi.md#logout_logout) | **POST** /auth/session/logout | Logout
*SessionApi* | [**revoke_all_revoke_all**](docs/SessionApi.md#revoke_all_revoke_all) | **DELETE** /auth/session/all | Delete All Sessions
*SessionApi* | [**revoke_revoke**](docs/SessionApi.md#revoke_revoke) | **DELETE** /auth/session/{id} | Revoke Session
*SyncApi* | [**get_settings_req**](docs/SyncApi.md#get_settings_req) | **POST** /sync/settings/fetch | Fetch Settings
*SyncApi* | [**get_unreads_req**](docs/SyncApi.md#get_unreads_req) | **GET** /sync/unreads | Fetch Unreads
*SyncApi* | [**set_settings_req**](docs/SyncApi.md#set_settings_req) | **POST** /sync/settings/set | Set Settings
*UserInformationApi* | [**change_username_req**](docs/UserInformationApi.md#change_username_req) | **PATCH** /users/@me/username | Change Username
*UserInformationApi* | [**edit_user_req**](docs/UserInformationApi.md#edit_user_req) | **PATCH** /users/@me | Edit User
*UserInformationApi* | [**fetch_profile_req**](docs/UserInformationApi.md#fetch_profile_req) | **GET** /users/{target}/profile | Fetch User Profile
*UserInformationApi* | [**fetch_self_req**](docs/UserInformationApi.md#fetch_self_req) | **GET** /users/@me | Fetch Self
*UserInformationApi* | [**fetch_user_req**](docs/UserInformationApi.md#fetch_user_req) | **GET** /users/{target} | Fetch User
*UserInformationApi* | [**get_default_avatar_req**](docs/UserInformationApi.md#get_default_avatar_req) | **GET** /users/{target}/default_avatar | Fetch Default Avatar
*VoiceApi* | [**voice_join_req**](docs/VoiceApi.md#voice_join_req) | **POST** /channels/{target}/join_call | Join Call
*WebPushApi* | [**subscribe_req**](docs/WebPushApi.md#subscribe_req) | **POST** /push/subscribe | Push Subscribe
*WebPushApi* | [**unsubscribe_req**](docs/WebPushApi.md#unsubscribe_req) | **POST** /push/unsubscribe | Unsubscribe


## Documentation For Models

 - [AccountInfo](docs/AccountInfo.md)
 - [AllMemberResponse](docs/AllMemberResponse.md)
 - [BanListResult](docs/BanListResult.md)
 - [BandcampType](docs/BandcampType.md)
 - [BannedUser](docs/BannedUser.md)
 - [BannedUserAvatar](docs/BannedUserAvatar.md)
 - [Bot](docs/Bot.md)
 - [BotInformation](docs/BotInformation.md)
 - [BotResponse](docs/BotResponse.md)
 - [BotResponseBot](docs/BotResponseBot.md)
 - [BotResponseUser](docs/BotResponseUser.md)
 - [BulkMessageResponse](docs/BulkMessageResponse.md)
 - [BulkMessageResponseAnyOf](docs/BulkMessageResponseAnyOf.md)
 - [CaptchaFeature](docs/CaptchaFeature.md)
 - [Category](docs/Category.md)
 - [Channel](docs/Channel.md)
 - [ChannelCompositeKey](docs/ChannelCompositeKey.md)
 - [ChannelOneOf](docs/ChannelOneOf.md)
 - [ChannelOneOf1](docs/ChannelOneOf1.md)
 - [ChannelOneOf2](docs/ChannelOneOf2.md)
 - [ChannelOneOf3](docs/ChannelOneOf3.md)
 - [ChannelOneOf4](docs/ChannelOneOf4.md)
 - [ChannelType](docs/ChannelType.md)
 - [ChannelUnread](docs/ChannelUnread.md)
 - [ChannelUnreadId](docs/ChannelUnreadId.md)
 - [CreateServerResponse](docs/CreateServerResponse.md)
 - [CreateServerResponseServer](docs/CreateServerResponseServer.md)
 - [CreateVoiceUserResponse](docs/CreateVoiceUserResponse.md)
 - [Data](docs/Data.md)
 - [DataAccountDeletion](docs/DataAccountDeletion.md)
 - [DataBanCreate](docs/DataBanCreate.md)
 - [DataChangeEmail](docs/DataChangeEmail.md)
 - [DataChangePassword](docs/DataChangePassword.md)
 - [DataChangeUsername](docs/DataChangeUsername.md)
 - [DataCreateAccount](docs/DataCreateAccount.md)
 - [DataCreateBot](docs/DataCreateBot.md)
 - [DataCreateChannel](docs/DataCreateChannel.md)
 - [DataCreateEmoji](docs/DataCreateEmoji.md)
 - [DataCreateEmojiParent](docs/DataCreateEmojiParent.md)
 - [DataCreateGroup](docs/DataCreateGroup.md)
 - [DataCreateRole](docs/DataCreateRole.md)
 - [DataCreateServer](docs/DataCreateServer.md)
 - [DataDefaultChannelPermissions](docs/DataDefaultChannelPermissions.md)
 - [DataDefaultChannelPermissionsAnyOf](docs/DataDefaultChannelPermissionsAnyOf.md)
 - [DataDefaultChannelPermissionsAnyOf1](docs/DataDefaultChannelPermissionsAnyOf1.md)
 - [DataEditBot](docs/DataEditBot.md)
 - [DataEditChannel](docs/DataEditChannel.md)
 - [DataEditMessage](docs/DataEditMessage.md)
 - [DataEditRole](docs/DataEditRole.md)
 - [DataEditServer](docs/DataEditServer.md)
 - [DataEditServerSystemMessages](docs/DataEditServerSystemMessages.md)
 - [DataEditSession](docs/DataEditSession.md)
 - [DataEditUser](docs/DataEditUser.md)
 - [DataEditUserProfile](docs/DataEditUserProfile.md)
 - [DataEditUserStatus](docs/DataEditUserStatus.md)
 - [DataHello](docs/DataHello.md)
 - [DataLogin](docs/DataLogin.md)
 - [DataLoginAnyOf](docs/DataLoginAnyOf.md)
 - [DataLoginAnyOf1](docs/DataLoginAnyOf1.md)
 - [DataMemberEdit](docs/DataMemberEdit.md)
 - [DataMessageSend](docs/DataMessageSend.md)
 - [DataMessageSendInteractions](docs/DataMessageSendInteractions.md)
 - [DataMessageSendMasquerade](docs/DataMessageSendMasquerade.md)
 - [DataOnboard](docs/DataOnboard.md)
 - [DataPasswordReset](docs/DataPasswordReset.md)
 - [DataPermissions](docs/DataPermissions.md)
 - [DataResendVerification](docs/DataResendVerification.md)
 - [DataSendFriendRequest](docs/DataSendFriendRequest.md)
 - [DataSendPasswordReset](docs/DataSendPasswordReset.md)
 - [DataSetServerDefaultPermission](docs/DataSetServerDefaultPermission.md)
 - [DataSetServerRolePermission](docs/DataSetServerRolePermission.md)
 - [DataSetServerRolePermissionPermissions](docs/DataSetServerRolePermissionPermissions.md)
 - [Embed](docs/Embed.md)
 - [EmbedOneOf](docs/EmbedOneOf.md)
 - [EmbedOneOf1](docs/EmbedOneOf1.md)
 - [EmbedOneOf2](docs/EmbedOneOf2.md)
 - [EmbedOneOf3](docs/EmbedOneOf3.md)
 - [EmbedOneOf4](docs/EmbedOneOf4.md)
 - [Emoji](docs/Emoji.md)
 - [EmojiParent](docs/EmojiParent.md)
 - [EmojiParentOneOf](docs/EmojiParentOneOf.md)
 - [EmojiParentOneOf1](docs/EmojiParentOneOf1.md)
 - [Error](docs/Error.md)
 - [ErrorOneOf](docs/ErrorOneOf.md)
 - [ErrorOneOf1](docs/ErrorOneOf1.md)
 - [ErrorOneOf10](docs/ErrorOneOf10.md)
 - [ErrorOneOf11](docs/ErrorOneOf11.md)
 - [ErrorOneOf12](docs/ErrorOneOf12.md)
 - [ErrorOneOf13](docs/ErrorOneOf13.md)
 - [ErrorOneOf14](docs/ErrorOneOf14.md)
 - [ErrorOneOf15](docs/ErrorOneOf15.md)
 - [ErrorOneOf16](docs/ErrorOneOf16.md)
 - [ErrorOneOf17](docs/ErrorOneOf17.md)
 - [ErrorOneOf18](docs/ErrorOneOf18.md)
 - [ErrorOneOf19](docs/ErrorOneOf19.md)
 - [ErrorOneOf2](docs/ErrorOneOf2.md)
 - [ErrorOneOf20](docs/ErrorOneOf20.md)
 - [ErrorOneOf21](docs/ErrorOneOf21.md)
 - [ErrorOneOf22](docs/ErrorOneOf22.md)
 - [ErrorOneOf23](docs/ErrorOneOf23.md)
 - [ErrorOneOf24](docs/ErrorOneOf24.md)
 - [ErrorOneOf25](docs/ErrorOneOf25.md)
 - [ErrorOneOf26](docs/ErrorOneOf26.md)
 - [ErrorOneOf27](docs/ErrorOneOf27.md)
 - [ErrorOneOf28](docs/ErrorOneOf28.md)
 - [ErrorOneOf29](docs/ErrorOneOf29.md)
 - [ErrorOneOf3](docs/ErrorOneOf3.md)
 - [ErrorOneOf30](docs/ErrorOneOf30.md)
 - [ErrorOneOf31](docs/ErrorOneOf31.md)
 - [ErrorOneOf32](docs/ErrorOneOf32.md)
 - [ErrorOneOf33](docs/ErrorOneOf33.md)
 - [ErrorOneOf34](docs/ErrorOneOf34.md)
 - [ErrorOneOf35](docs/ErrorOneOf35.md)
 - [ErrorOneOf36](docs/ErrorOneOf36.md)
 - [ErrorOneOf37](docs/ErrorOneOf37.md)
 - [ErrorOneOf38](docs/ErrorOneOf38.md)
 - [ErrorOneOf39](docs/ErrorOneOf39.md)
 - [ErrorOneOf4](docs/ErrorOneOf4.md)
 - [ErrorOneOf40](docs/ErrorOneOf40.md)
 - [ErrorOneOf41](docs/ErrorOneOf41.md)
 - [ErrorOneOf42](docs/ErrorOneOf42.md)
 - [ErrorOneOf43](docs/ErrorOneOf43.md)
 - [ErrorOneOf44](docs/ErrorOneOf44.md)
 - [ErrorOneOf45](docs/ErrorOneOf45.md)
 - [ErrorOneOf5](docs/ErrorOneOf5.md)
 - [ErrorOneOf6](docs/ErrorOneOf6.md)
 - [ErrorOneOf7](docs/ErrorOneOf7.md)
 - [ErrorOneOf8](docs/ErrorOneOf8.md)
 - [ErrorOneOf9](docs/ErrorOneOf9.md)
 - [Feature](docs/Feature.md)
 - [FieldsBot](docs/FieldsBot.md)
 - [FieldsChannel](docs/FieldsChannel.md)
 - [FieldsMember](docs/FieldsMember.md)
 - [FieldsRole](docs/FieldsRole.md)
 - [FieldsServer](docs/FieldsServer.md)
 - [FieldsUser](docs/FieldsUser.md)
 - [File](docs/File.md)
 - [FileMetadata](docs/FileMetadata.md)
 - [Image](docs/Image.md)
 - [ImageSize](docs/ImageSize.md)
 - [Interactions](docs/Interactions.md)
 - [Invite](docs/Invite.md)
 - [InviteBotDestination](docs/InviteBotDestination.md)
 - [InviteBotDestinationAnyOf](docs/InviteBotDestinationAnyOf.md)
 - [InviteBotDestinationAnyOf1](docs/InviteBotDestinationAnyOf1.md)
 - [InviteJoinResponse](docs/InviteJoinResponse.md)
 - [InviteJoinResponseOneOf](docs/InviteJoinResponseOneOf.md)
 - [InviteOneOf](docs/InviteOneOf.md)
 - [InviteOneOf1](docs/InviteOneOf1.md)
 - [InviteResponse](docs/InviteResponse.md)
 - [InviteResponseOneOf](docs/InviteResponseOneOf.md)
 - [InviteResponseOneOf1](docs/InviteResponseOneOf1.md)
 - [LightspeedType](docs/LightspeedType.md)
 - [Masquerade](docs/Masquerade.md)
 - [Member](docs/Member.md)
 - [MemberCompositeKey](docs/MemberCompositeKey.md)
 - [MemberId](docs/MemberId.md)
 - [Message](docs/Message.md)
 - [MessageInteractions](docs/MessageInteractions.md)
 - [MessageMasquerade](docs/MessageMasquerade.md)
 - [MessageSort](docs/MessageSort.md)
 - [MessageSystem](docs/MessageSystem.md)
 - [Metadata](docs/Metadata.md)
 - [MetadataOneOf](docs/MetadataOneOf.md)
 - [MetadataOneOf1](docs/MetadataOneOf1.md)
 - [MetadataOneOf2](docs/MetadataOneOf2.md)
 - [MetadataOneOf3](docs/MetadataOneOf3.md)
 - [MetadataOneOf4](docs/MetadataOneOf4.md)
 - [MfaMethod](docs/MfaMethod.md)
 - [MfaResponse](docs/MfaResponse.md)
 - [MfaResponseAnyOf](docs/MfaResponseAnyOf.md)
 - [MfaResponseAnyOf1](docs/MfaResponseAnyOf1.md)
 - [MfaResponseAnyOf2](docs/MfaResponseAnyOf2.md)
 - [MfaTicket](docs/MfaTicket.md)
 - [MultiFactorStatus](docs/MultiFactorStatus.md)
 - [MutualResponse](docs/MutualResponse.md)
 - [NewRoleResponse](docs/NewRoleResponse.md)
 - [NewRoleResponseRole](docs/NewRoleResponseRole.md)
 - [OptionsBulkDelete](docs/OptionsBulkDelete.md)
 - [OptionsFetchSettings](docs/OptionsFetchSettings.md)
 - [OptionsMessageSearch](docs/OptionsMessageSearch.md)
 - [OptionsQueryStale](docs/OptionsQueryStale.md)
 - [Override](docs/Override.md)
 - [OverrideField](docs/OverrideField.md)
 - [OwnedBotsResponse](docs/OwnedBotsResponse.md)
 - [Permission](docs/Permission.md)
 - [Presence](docs/Presence.md)
 - [PublicBot](docs/PublicBot.md)
 - [PublicBotAvatar](docs/PublicBotAvatar.md)
 - [RAuthError](docs/RAuthError.md)
 - [RAuthErrorOneOf](docs/RAuthErrorOneOf.md)
 - [RAuthErrorOneOf1](docs/RAuthErrorOneOf1.md)
 - [RAuthErrorOneOf10](docs/RAuthErrorOneOf10.md)
 - [RAuthErrorOneOf11](docs/RAuthErrorOneOf11.md)
 - [RAuthErrorOneOf12](docs/RAuthErrorOneOf12.md)
 - [RAuthErrorOneOf13](docs/RAuthErrorOneOf13.md)
 - [RAuthErrorOneOf14](docs/RAuthErrorOneOf14.md)
 - [RAuthErrorOneOf15](docs/RAuthErrorOneOf15.md)
 - [RAuthErrorOneOf16](docs/RAuthErrorOneOf16.md)
 - [RAuthErrorOneOf2](docs/RAuthErrorOneOf2.md)
 - [RAuthErrorOneOf3](docs/RAuthErrorOneOf3.md)
 - [RAuthErrorOneOf4](docs/RAuthErrorOneOf4.md)
 - [RAuthErrorOneOf5](docs/RAuthErrorOneOf5.md)
 - [RAuthErrorOneOf6](docs/RAuthErrorOneOf6.md)
 - [RAuthErrorOneOf7](docs/RAuthErrorOneOf7.md)
 - [RAuthErrorOneOf8](docs/RAuthErrorOneOf8.md)
 - [RAuthErrorOneOf9](docs/RAuthErrorOneOf9.md)
 - [Relationship](docs/Relationship.md)
 - [RelationshipStatus](docs/RelationshipStatus.md)
 - [Reply](docs/Reply.md)
 - [ResponseLogin](docs/ResponseLogin.md)
 - [ResponseLoginOneOf](docs/ResponseLoginOneOf.md)
 - [ResponseLoginOneOf1](docs/ResponseLoginOneOf1.md)
 - [ResponseTotpSecret](docs/ResponseTotpSecret.md)
 - [RevoltConfig](docs/RevoltConfig.md)
 - [RevoltConfigFeatures](docs/RevoltConfigFeatures.md)
 - [RevoltFeatures](docs/RevoltFeatures.md)
 - [RevoltFeaturesAutumn](docs/RevoltFeaturesAutumn.md)
 - [RevoltFeaturesCaptcha](docs/RevoltFeaturesCaptcha.md)
 - [RevoltFeaturesJanuary](docs/RevoltFeaturesJanuary.md)
 - [RevoltFeaturesVoso](docs/RevoltFeaturesVoso.md)
 - [Role](docs/Role.md)
 - [RolePermissions](docs/RolePermissions.md)
 - [SendableEmbed](docs/SendableEmbed.md)
 - [Server](docs/Server.md)
 - [ServerBan](docs/ServerBan.md)
 - [ServerBanner](docs/ServerBanner.md)
 - [ServerIcon](docs/ServerIcon.md)
 - [ServerSystemMessages](docs/ServerSystemMessages.md)
 - [SessionInfo](docs/SessionInfo.md)
 - [Special](docs/Special.md)
 - [SpecialOneOf](docs/SpecialOneOf.md)
 - [SpecialOneOf1](docs/SpecialOneOf1.md)
 - [SpecialOneOf2](docs/SpecialOneOf2.md)
 - [SpecialOneOf3](docs/SpecialOneOf3.md)
 - [SpecialOneOf4](docs/SpecialOneOf4.md)
 - [SpecialOneOf5](docs/SpecialOneOf5.md)
 - [SpecialOneOf6](docs/SpecialOneOf6.md)
 - [SpecialOneOf7](docs/SpecialOneOf7.md)
 - [SystemMessage](docs/SystemMessage.md)
 - [SystemMessageChannels](docs/SystemMessageChannels.md)
 - [SystemMessageOneOf](docs/SystemMessageOneOf.md)
 - [SystemMessageOneOf1](docs/SystemMessageOneOf1.md)
 - [SystemMessageOneOf10](docs/SystemMessageOneOf10.md)
 - [SystemMessageOneOf2](docs/SystemMessageOneOf2.md)
 - [SystemMessageOneOf3](docs/SystemMessageOneOf3.md)
 - [SystemMessageOneOf4](docs/SystemMessageOneOf4.md)
 - [SystemMessageOneOf5](docs/SystemMessageOneOf5.md)
 - [SystemMessageOneOf6](docs/SystemMessageOneOf6.md)
 - [SystemMessageOneOf7](docs/SystemMessageOneOf7.md)
 - [SystemMessageOneOf8](docs/SystemMessageOneOf8.md)
 - [SystemMessageOneOf9](docs/SystemMessageOneOf9.md)
 - [TwitchType](docs/TwitchType.md)
 - [User](docs/User.md)
 - [UserAvatar](docs/UserAvatar.md)
 - [UserBot](docs/UserBot.md)
 - [UserPermission](docs/UserPermission.md)
 - [UserProfile](docs/UserProfile.md)
 - [UserProfileBackground](docs/UserProfileBackground.md)
 - [UserProfileData](docs/UserProfileData.md)
 - [UserStatus](docs/UserStatus.md)
 - [Video](docs/Video.md)
 - [VoiceFeature](docs/VoiceFeature.md)
 - [WebPushSubscription](docs/WebPushSubscription.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

contact@revolt.chat

