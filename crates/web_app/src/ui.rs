#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
    pub compact: bool,
}

impl Viewport {
    pub const fn desktop() -> Self {
        Self {
            width: 960.0,
            height: 540.0,
            compact: false,
        }
    }

    pub const fn compact() -> Self {
        Self {
            width: 720.0,
            height: 960.0,
            compact: true,
        }
    }

    pub fn choose(window_width: f64, window_height: f64) -> Self {
        if window_width < 860.0 || window_height > window_width {
            Self::compact()
        } else {
            Self::desktop()
        }
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::desktop()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x
            && x <= self.x + self.width
            && y >= self.y
            && y <= self.y + self.height
    }

    pub fn center_x(&self) -> f64 {
        self.x + self.width / 2.0
    }

    pub fn center_y(&self) -> f64 {
        self.y + self.height / 2.0
    }
}

pub struct MenuLayout {
    pub difficulty_cards: Vec<Rect>,
    pub variety_cards: Vec<Rect>,
    pub start_button: Rect,
}

pub fn menu_layout(viewport: Viewport) -> MenuLayout {
    if viewport.compact {
        let card_width = viewport.width - 72.0;
        let card_height = 70.0;
        let gap = 16.0;
        let difficulty_cards = option_stack(36.0, 178.0, card_width, card_height, gap, 3);
        let variety_cards = option_stack(
            36.0,
            difficulty_cards
                .last()
                .map(|card| card.y + card.height + 64.0)
                .unwrap_or(480.0),
            card_width,
            card_height,
            gap,
            3,
        );

        MenuLayout {
            difficulty_cards,
            variety_cards,
            start_button: Rect {
                x: 72.0,
                y: viewport.height - 92.0,
                width: viewport.width - 144.0,
                height: 54.0,
            },
        }
    } else {
        let side_padding = 88.0;
        let column_gap = 28.0;
        let card_width = (viewport.width - side_padding * 2.0 - column_gap) / 2.0;
        let card_height = 68.0;
        let gap = 18.0;
        let difficulty_cards = option_stack(side_padding, 188.0, card_width, card_height, gap, 3);
        let variety_x = side_padding + card_width + column_gap;
        let variety_cards = option_stack(variety_x, 188.0, card_width, card_height, gap, 3);

        MenuLayout {
            difficulty_cards,
            variety_cards,
            start_button: Rect {
                x: (viewport.width - 260.0) / 2.0,
                y: viewport.height - 78.0,
                width: 260.0,
                height: 46.0,
            },
        }
    }
}

fn option_stack(x: f64, start_y: f64, width: f64, height: f64, gap: f64, count: usize) -> Vec<Rect> {
    (0..count)
        .map(|index| Rect {
            x,
            y: start_y + index as f64 * (height + gap),
            width,
            height,
        })
        .collect()
}

pub struct PlayLayout {
    pub fall_zone: Rect,
    pub bucket_rects: Vec<Rect>,
    pub lane_hit_rects: Vec<Rect>,
    pub collection_panel: Rect,
    pub pause_button: Rect,
    pub game_center_x: f64,
    pub controls_y: f64,
}

pub fn play_layout(viewport: Viewport, lane_count: usize) -> PlayLayout {
    let lane_count = lane_count.max(1);
    if viewport.compact {
        let side_padding = 28.0;
        let lane_gap = 12.0;
        let bucket_height = 108.0;
        let collection_height = 164.0;
        let collection_panel = Rect {
            x: side_padding,
            y: viewport.height - collection_height - 72.0,
            width: viewport.width - side_padding * 2.0,
            height: collection_height,
        };
        let bucket_top = collection_panel.y - bucket_height - 24.0;
        let fall_zone = Rect {
            x: side_padding,
            y: 120.0,
            width: viewport.width - side_padding * 2.0,
            height: bucket_top - 120.0,
        };
        let lane_width =
            (fall_zone.width - lane_gap * (lane_count.saturating_sub(1)) as f64) / lane_count as f64;

        let bucket_rects: Vec<Rect> = (0..lane_count)
            .map(|index| Rect {
                x: fall_zone.x + index as f64 * (lane_width + lane_gap),
                y: bucket_top,
                width: lane_width,
                height: bucket_height,
            })
            .collect();
        let lane_hit_rects = bucket_rects
            .iter()
            .map(|bucket| Rect {
                x: bucket.x,
                y: fall_zone.y,
                width: bucket.width,
                height: bucket.y + bucket.height - fall_zone.y,
            })
            .collect();

        PlayLayout {
            fall_zone,
            bucket_rects,
            lane_hit_rects,
            collection_panel,
            pause_button: Rect {
                x: viewport.width - side_padding - 100.0,
                y: 20.0,
                width: 100.0,
                height: 38.0,
            },
            game_center_x: viewport.width / 2.0,
            controls_y: viewport.height - 18.0,
        }
    } else {
        let outer_padding = 24.0;
        let panel_width = 216.0;
        let column_gap = 18.0;
        let lane_gap = 10.0;
        let playable_width = viewport.width - outer_padding * 2.0 - panel_width - column_gap;
        let lane_width =
            (playable_width - lane_gap * (lane_count.saturating_sub(1)) as f64) / lane_count as f64;
        let fall_zone = Rect {
            x: outer_padding,
            y: 74.0,
            width: playable_width,
            height: viewport.height - 74.0 - 88.0 - 40.0,
        };
        let bucket_rects: Vec<Rect> = (0..lane_count)
            .map(|index| Rect {
                x: fall_zone.x + index as f64 * (lane_width + lane_gap),
                y: fall_zone.y + fall_zone.height,
                width: lane_width,
                height: 88.0,
            })
            .collect();
        let lane_hit_rects = bucket_rects
            .iter()
            .map(|bucket| Rect {
                x: bucket.x,
                y: fall_zone.y,
                width: bucket.width,
                height: bucket.y + bucket.height - fall_zone.y,
            })
            .collect();
        let collection_panel = Rect {
            x: outer_padding + playable_width + column_gap,
            y: 74.0,
            width: panel_width,
            height: viewport.height - 108.0,
        };

        PlayLayout {
            fall_zone,
            bucket_rects,
            lane_hit_rects,
            collection_panel,
            pause_button: Rect {
                x: viewport.width - outer_padding - 96.0,
                y: 18.0,
                width: 96.0,
                height: 32.0,
            },
            game_center_x: outer_padding + playable_width / 2.0,
            controls_y: viewport.height - 10.0,
        }
    }
}

pub fn primary_button_rect(viewport: Viewport) -> Rect {
    let width = if viewport.compact {
        viewport.width - 144.0
    } else {
        280.0
    };
    let height = if viewport.compact { 56.0 } else { 48.0 };
    Rect {
        x: (viewport.width - width) / 2.0,
        y: viewport.height - if viewport.compact { 92.0 } else { 74.0 },
        width,
        height,
    }
}

pub fn secondary_button_rect(viewport: Viewport) -> Rect {
    let width = if viewport.compact {
        viewport.width - 144.0
    } else {
        280.0
    };
    let height = if viewport.compact { 52.0 } else { 44.0 };
    Rect {
        x: (viewport.width - width) / 2.0,
        y: viewport.height - if viewport.compact { 156.0 } else { 130.0 },
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::{menu_layout, play_layout, primary_button_rect, Viewport};

    #[test]
    fn chooses_compact_viewport_for_portrait_windows() {
        let viewport = Viewport::choose(390.0, 844.0);
        assert!(viewport.compact);
        assert_eq!(viewport.width, 720.0);
    }

    #[test]
    fn menu_layout_exposes_clickable_cards_and_button() {
        let layout = menu_layout(Viewport::compact());
        assert_eq!(layout.difficulty_cards.len(), 3);
        assert_eq!(layout.variety_cards.len(), 3);
        assert!(layout.start_button.width > 0.0);
        assert!(layout.start_button.y > layout.variety_cards[2].y);
    }

    #[test]
    fn compact_play_layout_moves_collection_below_buckets() {
        let layout = play_layout(Viewport::compact(), 4);
        assert_eq!(layout.bucket_rects.len(), 4);
        assert!(layout.collection_panel.y > layout.bucket_rects[0].y);
    }

    #[test]
    fn primary_button_stays_within_viewport() {
        let viewport = Viewport::desktop();
        let button = primary_button_rect(viewport);
        assert!(button.x >= 0.0);
        assert!(button.x + button.width <= viewport.width);
    }
}
