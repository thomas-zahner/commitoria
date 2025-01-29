use crate::types::ContributionActivity;

pub struct SvgRenderer {}

impl SvgRenderer {
    pub fn render(activity: &ContributionActivity) -> String {
        const WIDTH: u16 = 864;
        const HEIGHT: u16 = 140;

        format!(
            r#"<svg width="{}" height="{}" class="contrib-calendar" data-testid="contrib-calendar">
</svg>"#,
            WIDTH, HEIGHT
        )
    }
}

#[cfg(test)]
mod tests {
    use super::SvgRenderer;
    use std::collections::BTreeMap;
    use time::Date;

    #[test]
    fn basic() {
        let first = Date::from_calendar_date(2024, time::Month::January, 1).unwrap();
        let activity = BTreeMap::from([(first, 1)]);
        let svg = SvgRenderer::render(&activity.into());
        assert_eq!(
            &svg,
            r#"<svg width="864" height="140" class="contrib-calendar" data-testid="contrib-calendar">
</svg>"#
        )
    }
}
