use crate::{
    channels::Channels,
    gui::{
        components::{
            AnnouncementPanelComponent, AnnouncementPanelMessage,
            ChangelogPanelComponent, ChangelogPanelMessage, CommunityShowcaseComponent,
            CommunityShowcasePanelMessage, GamePanelComponent, GamePanelMessage,
            LogoPanelComponent, NewsPanelComponent, NewsPanelMessage,
            SettingsPanelComponent, SettingsPanelMessage,
        },
        rss_feed::RssFeedComponentMessage::UpdateRssFeed,
        style,
        views::Action,
        widget::*,
    },
    profiles::Profile,
};

use iced::{
    Length, Task,
    widget::{column, container, row},
};

#[cfg(windows)]
use crate::gui::Result;

#[derive(Default, Debug, Clone)]
pub struct DefaultView {
    changelog_panel_component: ChangelogPanelComponent,
    announcement_panel_component: AnnouncementPanelComponent,
    logo_panel_component: LogoPanelComponent,
    community_showcase_component: CommunityShowcaseComponent,
    game_panel_component: GamePanelComponent,
    news_panel_component: NewsPanelComponent,
    settings_panel_component: SettingsPanelComponent,
    show_settings: bool,
}

#[derive(Clone, Debug)]
pub enum DefaultViewMessage {
    // Messages
    Action(Action),
    Query,

    #[cfg(windows)]
    LauncherUpdate(Result<Option<self_update::update::Release>>),

    // User Interactions
    Interaction(Interaction),

    // Panel-specific messages
    GamePanel(GamePanelMessage),
    ChangelogPanel(ChangelogPanelMessage),
    AnnouncementPanel(AnnouncementPanelMessage),
    CommunityShowcasePanel(CommunityShowcasePanelMessage),
    NewsPanel(NewsPanelMessage),
    SettingsPanel(SettingsPanelMessage),
}

#[derive(Debug, Clone)]
pub enum Interaction {
    SettingsPressed,
    OpenURL(String),
}

impl DefaultView {
    pub fn subscription(&self) -> iced::Subscription<DefaultViewMessage> {
        self.game_panel_component
            .subscription()
            .map(DefaultViewMessage::GamePanel)
    }

    pub fn view<'a>(
        &'a self,
        active_profile: &'a Profile,
    ) -> Element<'a, DefaultViewMessage> {
        let Self {
            changelog_panel_component,
            announcement_panel_component,
            news_panel_component,
            logo_panel_component,
            community_showcase_component,
            game_panel_component,
            settings_panel_component,
            ..
        } = self;

        let left_middle_contents = if self.show_settings {
            settings_panel_component.view(active_profile)
        } else {
            community_showcase_component.view()
        };

        let left = container(
            column![]
                .push(container(logo_panel_component.view()).height(Length::Fill))
                .push(container(left_middle_contents).height(Length::Shrink))
                .push(
                    container(game_panel_component.view(active_profile))
                        .height(Length::Shrink),
                ),
        )
        .height(Length::Fill)
        .width(Length::Fixed(360.0))
        .style(style::container::sidepanel);

        let middle = container(
            column![]
                .push(
                    container(announcement_panel_component.view()).height(Length::Shrink),
                )
                .push(container(changelog_panel_component.view()).height(Length::Fill)),
        )
        .height(Length::Fill)
        .width(Length::Fill);
        let right = container(news_panel_component.view())
            .height(Length::Fill)
            .width(Length::Fixed(248.0))
            .style(style::container::sidepanel);

        let main_row = row![].push(left).push(middle).push(right);

        container(main_row)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(
        &mut self,
        msg: DefaultViewMessage,
        active_profile: &Profile,
    ) -> Task<DefaultViewMessage> {
        match msg {
            // Messages
            // Will be handled by main view
            DefaultViewMessage::Action(_) => {},
            DefaultViewMessage::Query => {
                let api_version_url = active_profile.api_version_url();
                let announcement_url = active_profile.announcement_url();
                return Task::batch(vec![
                    Task::future(NewsPanelComponent::load_news()).then(
                        |(update, task)| {
                            Task::batch([
                                Task::done(DefaultViewMessage::NewsPanel(
                                    NewsPanelMessage::RssUpdate(UpdateRssFeed(update)),
                                )),
                                task,
                            ])
                        },
                    ),
                    Task::perform(ChangelogPanelComponent::load_changelog(), |update| {
                        DefaultViewMessage::ChangelogPanel(
                            ChangelogPanelMessage::LoadChangelog(update),
                        )
                    }),
                    Task::perform(
                        AnnouncementPanelComponent::fetch(
                            api_version_url,
                            announcement_url,
                        ),
                        |update| {
                            DefaultViewMessage::AnnouncementPanel(
                                AnnouncementPanelMessage::FetchAnnouncement(update),
                            )
                        },
                    ),
                    Task::future(CommunityShowcaseComponent::load_community_posts())
                        .then(|(update, task)| {
                            Task::batch([
                                Task::done(DefaultViewMessage::CommunityShowcasePanel(
                                    CommunityShowcasePanelMessage::RssUpdate(
                                        UpdateRssFeed(update),
                                    ),
                                )),
                                task,
                            ])
                        }),
                    Task::perform(
                        Channels::fetch(active_profile.channel_url()),
                        |channels| {
                            DefaultViewMessage::SettingsPanel(
                                SettingsPanelMessage::ChannelsLoaded(channels),
                            )
                        },
                    ),
                    #[cfg(windows)]
                    Task::perform(
                        async { tokio::task::block_in_place(crate::windows::query) },
                        DefaultViewMessage::LauncherUpdate,
                    ),
                    Task::done(DefaultViewMessage::GamePanel(
                        GamePanelMessage::StartUpdate,
                    )),
                ]);
            },

            DefaultViewMessage::GamePanel(msg) => {
                if let Some(command) =
                    self.game_panel_component.update(msg, active_profile)
                {
                    return command;
                }
            },
            DefaultViewMessage::ChangelogPanel(msg) => {
                if let Some(command) = self.changelog_panel_component.update(msg) {
                    return command;
                }
            },
            DefaultViewMessage::AnnouncementPanel(msg) => {
                if let Some(command) = self.announcement_panel_component.update(msg) {
                    return command;
                }
            },
            DefaultViewMessage::CommunityShowcasePanel(msg) => {
                if let Some(command) = self.community_showcase_component.update(msg) {
                    return command;
                }
            },
            DefaultViewMessage::NewsPanel(msg) => {
                if let Some(command) = self.news_panel_component.update(msg) {
                    return command;
                }
            },

            DefaultViewMessage::SettingsPanel(msg) => {
                if let Some(command) =
                    self.settings_panel_component.update(msg, active_profile)
                {
                    return command;
                }
            },
            #[cfg(windows)]
            DefaultViewMessage::LauncherUpdate(update) => {
                if let Ok(Some(release)) = update {
                    return Task::perform(
                        async { Action::LauncherUpdate(release) },
                        DefaultViewMessage::Action,
                    );
                }
            },

            // User Interaction
            DefaultViewMessage::Interaction(interaction) => match interaction {
                Interaction::SettingsPressed => {
                    self.show_settings = !self.show_settings;
                },
                Interaction::OpenURL(url) => {
                    if let Err(e) = opener::open(url) {
                        tracing::error!(
                            "Failed to open gitlab changelog website: {:?}",
                            e
                        );
                    }
                },
            },
        }

        Task::none()
    }
}
