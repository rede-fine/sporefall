use web_sys::CanvasRenderingContext2d;

use crate::app::{AppState, CenterAnimation, Difficulty, GamePhase};
use crate::catalog::Variety;
use crate::images::ImageCache;
use crate::ui::{self, Rect};

pub fn draw(ctx: &CanvasRenderingContext2d, app: &AppState, images: &ImageCache) {
    let viewport = app.viewport;
    let dpr = app.dpr;
    ctx.set_transform(dpr, 0.0, 0.0, dpr, 0.0, 0.0).unwrap_or(());
    ctx.set_fill_style_str("#1a2218");
    ctx.fill_rect(0.0, 0.0, viewport.width, viewport.height);

    match &app.phase {
        GamePhase::Menu => draw_menu(ctx, app),
        GamePhase::Playing => {
            draw_playing(ctx, app, images);
            draw_center_animation(ctx, app);
        }
        GamePhase::Paused => {
            draw_playing(ctx, app, images);
            draw_pause_overlay(ctx, app);
        }
        GamePhase::BasketFact { mushrooms, fact } => {
            draw_playing(ctx, app, images);
            draw_basket_fact(ctx, app, mushrooms, fact, images);
        }
        GamePhase::LevelComplete => draw_level_complete(ctx, app, images),
        GamePhase::GameOver => draw_game_over(ctx, app, images),
    }
}

fn draw_menu(ctx: &CanvasRenderingContext2d, app: &AppState) {
    let viewport = app.viewport;
    let layout = ui::menu_layout(viewport);
    let title_x = if viewport.compact { 54.0 } else { 96.0 };
    let title_y = if viewport.compact { 86.0 } else { 84.0 };

    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font(if viewport.compact {
        "bold 54px Georgia"
    } else {
        "bold 52px Georgia"
    });
    let _ = ctx.fill_text("Sporefall", title_x, title_y);

    ctx.set_fill_style_str("#a8c49a");
    ctx.set_font(if viewport.compact {
        "20px Georgia"
    } else {
        "18px Georgia"
    });
    let _ = ctx.fill_text(
        "Sort mushroom species into the right buckets.",
        title_x,
        title_y + if viewport.compact { 34.0 } else { 30.0 },
    );

    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font(if viewport.compact {
        "bold 26px Georgia"
    } else {
        "bold 22px Georgia"
    });
    let _ = ctx.fill_text(
        "Game Mode",
        layout.difficulty_cards[0].x,
        layout.difficulty_cards[0].y - if viewport.compact { 24.0 } else { 20.0 },
    );
    let _ = ctx.fill_text(
        "Variety",
        layout.variety_cards[0].x,
        layout.variety_cards[0].y - if viewport.compact { 24.0 } else { 20.0 },
    );

    for (index, diff) in Difficulty::all().iter().enumerate() {
        draw_menu_option(
            ctx,
            &layout.difficulty_cards[index],
            diff.label(),
            diff.description(),
            app.menu_column == 0 && index == app.menu_selection,
            index == app.menu_selection,
            viewport.compact,
        );
    }

    for (index, variety) in Variety::all().iter().enumerate() {
        draw_menu_option(
            ctx,
            &layout.variety_cards[index],
            variety.label(),
            variety.description(),
            app.menu_column == 1 && index == app.variety_selection,
            index == app.variety_selection,
            viewport.compact,
        );
    }

    let start_label = format!(
        "Start {} / {}",
        Difficulty::all()[app.menu_selection].label(),
        Variety::all()[app.variety_selection].label()
    );
    draw_button(
        ctx,
        &layout.start_button,
        &start_label,
        "#f5d67a",
        "#203122",
        viewport.compact,
    );

    let level_lines = if viewport.compact {
        vec![
            "4 levels: Cap Color -> Culinary Type".to_owned(),
            "Ecological Role -> Peak Season".to_owned(),
        ]
    } else {
        vec!["4 levels: Cap Color -> Culinary Type -> Ecological Role -> Peak Season".to_owned()]
    };
    ctx.set_fill_style_str("#7ca17e");
    ctx.set_font(if viewport.compact {
        "18px Georgia"
    } else {
        "14px Georgia"
    });
    let mut info_y = layout.start_button.y - if viewport.compact { 54.0 } else { 28.0 };
    for line in level_lines {
        let _ = ctx.fill_text(&line, title_x, info_y);
        info_y += if viewport.compact { 24.0 } else { 18.0 };
    }

    if let Some(summary) = app.menu_import_summary() {
        ctx.set_fill_style_str("#f5d67a");
        ctx.set_font(if viewport.compact {
            "15px Georgia"
        } else {
            "12px Georgia"
        });
        let import_y = layout.start_button.y - if viewport.compact { 88.0 } else { 50.0 };
        let _ = ctx.fill_text(&summary, title_x, import_y);
    }

    ctx.set_fill_style_str("#a8c49a");
    ctx.set_font(if viewport.compact {
        "18px Georgia"
    } else {
        "15px Georgia"
    });
    let controls = if viewport.compact {
        "Tap cards to choose. Keyboard: arrows + Enter/Space."
    } else {
        "Mouse/touch: click cards + Start. Keyboard: arrows + Enter/Space."
    };
    let _ = ctx.fill_text(controls, title_x, viewport.height - 18.0);
}

fn draw_menu_option(
    ctx: &CanvasRenderingContext2d,
    rect: &Rect,
    title: &str,
    subtitle: &str,
    selected: bool,
    active: bool,
    compact: bool,
) {
    let fill = if selected {
        "#3a5a3c"
    } else if active {
        "#2a3a2c"
    } else {
        "#223024"
    };
    ctx.set_fill_style_str(fill);
    ctx.fill_rect(rect.x, rect.y, rect.width, rect.height);
    ctx.set_stroke_style_str(if selected { "#f5d67a" } else { "#4a6a4c" });
    ctx.set_line_width(if selected { 3.0 } else { 1.5 });
    ctx.stroke_rect(rect.x, rect.y, rect.width, rect.height);

    ctx.set_fill_style_str(if active { "#f5d67a" } else { "#f8f3e8" });
    ctx.set_font(if compact {
        "bold 24px Georgia"
    } else {
        "bold 21px Georgia"
    });
    let _ = ctx.fill_text(title, rect.x + 18.0, rect.y + if compact { 28.0 } else { 24.0 });

    let wrapped = wrap_text(subtitle, if compact { 38 } else { 34 });
    ctx.set_fill_style_str("#a8c49a");
    ctx.set_font(if compact {
        "16px Georgia"
    } else {
        "13px Georgia"
    });
    for (index, line) in wrapped.iter().take(if compact { 2 } else { 2 }).enumerate() {
        let _ = ctx.fill_text(
            line,
            rect.x + 18.0,
            rect.y + if compact { 50.0 } else { 44.0 } + index as f64 * if compact { 18.0 } else { 14.0 },
        );
    }
}

fn draw_playing(ctx: &CanvasRenderingContext2d, app: &AppState, images: &ImageCache) {
    let viewport = app.viewport;
    let layout = ui::play_layout(viewport, app.game.lane_count());
    let bucket_top = layout.bucket_rects.first().map(|rect| rect.y).unwrap_or(layout.fall_zone.y);
    let bucket_bottom = layout
        .bucket_rects
        .first()
        .map(|rect| rect.y + rect.height)
        .unwrap_or(bucket_top);

    draw_button(
        ctx,
        &layout.pause_button,
        "Pause",
        "#2a3a2c",
        "#f8f3e8",
        viewport.compact,
    );

    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font(if viewport.compact {
        "bold 28px Georgia"
    } else {
        "bold 16px Georgia"
    });
    let _ = ctx.fill_text(
        &format!("Level {} - {}", app.current_level + 1, app.category_mode.display_name()),
        layout.fall_zone.x,
        if viewport.compact { 40.0 } else { 22.0 },
    );

    let stats_y = if viewport.compact { 70.0 } else { 42.0 };
    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font(if viewport.compact {
        "bold 18px Georgia"
    } else {
        "bold 12px Georgia"
    });
    let _ = ctx.fill_text(&format!("Score {}", app.game.score()), layout.fall_zone.x, stats_y);
    let _ = ctx.fill_text(
        &format!("Baskets {}", app.total_baskets),
        layout.fall_zone.x + if viewport.compact { 150.0 } else { 100.0 },
        stats_y,
    );
    let _ = ctx.fill_text(
        &format!("Sorted {}/{}", app.sorted_this_level.len(), app.active_catalog_len()),
        layout.fall_zone.x + if viewport.compact { 320.0 } else { 200.0 },
        stats_y,
    );

    ctx.set_fill_style_str("#8aaa7c");
    ctx.set_font(if viewport.compact {
        "16px Georgia"
    } else {
        "11px Georgia"
    });
    let badge_y = if viewport.compact { 96.0 } else { 58.0 };
    let _ = ctx.fill_text(
        &format!(
            "[{} / {} / {}]",
            app.difficulty.label(),
            app.variety.label(),
            app.active_source_label()
        ),
        layout.fall_zone.x,
        badge_y,
    );

    if let Some(summary) = app.active_source_summary() {
        ctx.set_fill_style_str("#d7c486");
        ctx.set_font(if viewport.compact {
            "14px Georgia"
        } else {
            "11px Georgia"
        });
        let _ = ctx.fill_text(
            &summary,
            layout.fall_zone.x + if viewport.compact { 0.0 } else { 180.0 },
            if viewport.compact { 92.0 } else { 58.0 },
        );
    }

    if let Some(message) = app.feedback_message() {
        ctx.set_fill_style_str("#d7c486");
        ctx.set_font(if viewport.compact {
            "16px Georgia"
        } else {
            "12px Georgia"
        });
        let message_width = if viewport.compact { 54 } else { 46 };
        let mut y = if viewport.compact { badge_y + 24.0 } else { 22.0 };
        for line in wrap_text(message, message_width).into_iter().take(if viewport.compact { 2 } else { 1 }) {
            let _ = ctx.fill_text(
                &line,
                if viewport.compact {
                    layout.fall_zone.x
                } else {
                    layout.fall_zone.x + 300.0
                },
                y,
            );
            y += 18.0;
        }
    }

    ctx.set_fill_style_str("#1e2e1c");
    ctx.fill_rect(
        layout.fall_zone.x,
        layout.fall_zone.y,
        layout.fall_zone.width,
        layout.fall_zone.height,
    );
    ctx.set_stroke_style_str("#334736");
    ctx.set_line_width(2.0);
    ctx.stroke_rect(
        layout.fall_zone.x,
        layout.fall_zone.y,
        layout.fall_zone.width,
        layout.fall_zone.height,
    );

    let labels = app.category_mode.labels();
    for (index, bucket) in layout.bucket_rects.iter().enumerate() {
        ctx.set_fill_style_str("#2a3a2c");
        ctx.fill_rect(bucket.x, bucket.y, bucket.width, bucket.height);
        ctx.set_stroke_style_str("#4a6a4c");
        ctx.set_line_width(2.0);
        ctx.stroke_rect(bucket.x, bucket.y, bucket.width, bucket.height);

        ctx.set_fill_style_str("#a8c49a");
        ctx.set_font(if viewport.compact {
            "bold 16px Georgia"
        } else {
            "bold 11px Georgia"
        });
        let label = labels.get(index).copied().unwrap_or("???");
        let _ = ctx.fill_text(label, bucket.x + 8.0, bucket.y + if viewport.compact { 22.0 } else { 16.0 });

        let lane_stack = &app.game.lanes()[index];
        for (stack_index, mushroom) in lane_stack.iter().enumerate() {
            let piece_height = if viewport.compact { 26.0 } else { 20.0 };
            let piece_y = bucket_bottom - 6.0 - (stack_index as f64 + 1.0) * piece_height;
            if piece_y < bucket.y + if viewport.compact { 24.0 } else { 18.0 } {
                break;
            }
            draw_mushroom_item(
                ctx,
                mushroom,
                bucket.x + 6.0,
                piece_y,
                if viewport.compact { 22.0 } else { 16.0 },
                app.difficulty,
                images,
                true,
            );
        }

        ctx.set_stroke_style_str("#3a4a3c");
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.move_to(bucket.x, layout.fall_zone.y);
        ctx.line_to(bucket.x, bucket.y);
        ctx.move_to(bucket.x + bucket.width, layout.fall_zone.y);
        ctx.line_to(bucket.x + bucket.width, bucket.y);
        ctx.stroke();
    }

    if let Some(mushroom) = app.game.active_mushroom() {
        let active_lane = app.game.active_lane();
        if let Some(bucket) = layout.bucket_rects.get(active_lane) {
            let fall_y = layout.fall_zone.y + app.fall_progress * layout.fall_zone.height;
            let center_x = bucket.center_x();
            let img_size = (bucket.width * 0.75).min(if viewport.compact { 120.0 } else { 90.0 });

            draw_mushroom_item(
                ctx,
                mushroom,
                center_x - img_size / 2.0,
                fall_y - img_size / 2.0,
                img_size,
                app.difficulty,
                images,
                false,
            );

            match app.difficulty {
                Difficulty::Normal => {
                    ctx.set_fill_style_str("#f8f3e8");
                    ctx.set_font(if viewport.compact {
                        "bold 17px Georgia"
                    } else {
                        "bold 12px Georgia"
                    });
                    let name_width = mushroom.display_name.len() as f64 * if viewport.compact { 8.8 } else { 7.0 };
                    let _ = ctx.fill_text(
                        &mushroom.display_name,
                        center_x - name_width / 2.0,
                        fall_y + img_size / 2.0 + if viewport.compact { 20.0 } else { 14.0 },
                    );
                    ctx.set_fill_style_str("#a8c49a");
                    ctx.set_font(if viewport.compact {
                        "italic 15px Georgia"
                    } else {
                        "italic 10px Georgia"
                    });
                    let latin_width = mushroom.latin_name.len() as f64 * if viewport.compact { 6.4 } else { 5.5 };
                    let _ = ctx.fill_text(
                        &mushroom.latin_name,
                        center_x - latin_width / 2.0,
                        fall_y + img_size / 2.0 + if viewport.compact { 40.0 } else { 28.0 },
                    );
                }
                Difficulty::Tricky => {}
                Difficulty::Expert => {
                    ctx.set_fill_style_str("#a8c49a");
                    ctx.set_font(if viewport.compact {
                        "italic 16px Georgia"
                    } else {
                        "italic 11px Georgia"
                    });
                    let latin_width = mushroom.latin_name.len() as f64 * if viewport.compact { 6.4 } else { 5.5 };
                    let _ = ctx.fill_text(
                        &mushroom.latin_name,
                        center_x - latin_width / 2.0,
                        fall_y + img_size / 2.0 + if viewport.compact { 18.0 } else { 14.0 },
                    );
                }
            }

            ctx.set_stroke_style_str("#f5d67a");
            ctx.set_line_width(if viewport.compact { 4.0 } else { 3.0 });
            ctx.stroke_rect(bucket.x, bucket.y, bucket.width, bucket.height);

            ctx.set_fill_style_str("#f5d67a");
            ctx.set_font(if viewport.compact {
                "bold 18px Georgia"
            } else {
                "bold 14px Georgia"
            });
            let _ = ctx.fill_text("v", center_x - 6.0, bucket_top - 8.0);
        }
    }

    draw_basket_panel(ctx, app, images, &layout.collection_panel);

    ctx.set_fill_style_str("#5a7a5c");
    ctx.set_font(if viewport.compact {
        "15px Georgia"
    } else {
        "12px Georgia"
    });
    let controls = if viewport.compact {
        "Tap a lane to sort. Keyboard: arrows / Space / Esc."
    } else {
        "Mouse/touch: click a lane to sort. Keyboard: arrows / Space / Esc / N."
    };
    let _ = ctx.fill_text(controls, layout.fall_zone.x, layout.controls_y);
}

fn draw_basket_panel(
    ctx: &CanvasRenderingContext2d,
    app: &AppState,
    images: &ImageCache,
    panel: &Rect,
) {
    let viewport = app.viewport;
    ctx.set_fill_style_str("#1e2e1c");
    ctx.fill_rect(panel.x, panel.y, panel.width, panel.height);
    ctx.set_stroke_style_str("#3a5a3c");
    ctx.set_line_width(1.5);
    ctx.stroke_rect(panel.x, panel.y, panel.width, panel.height);

    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font(if viewport.compact {
        "bold 20px Georgia"
    } else {
        "bold 12px Georgia"
    });
    let _ = ctx.fill_text(
        "Basket Collection",
        panel.x + 10.0,
        panel.y + if viewport.compact { 26.0 } else { 18.0 },
    );

    if app.collected_mushrooms.is_empty() {
        ctx.set_fill_style_str("#8aaa7c");
        ctx.set_font(if viewport.compact {
            "16px Georgia"
        } else {
            "12px Georgia"
        });
        let empty_lines = if viewport.compact {
            vec![
                "Clear one mushroom in every lane to collect a basket.".to_owned(),
                "Basket rewards will appear here.".to_owned(),
            ]
        } else {
            vec!["Clear a full row to start filling your basket.".to_owned()]
        };
        let mut y = panel.y + if viewport.compact { 58.0 } else { 44.0 };
        for line in empty_lines {
            let _ = ctx.fill_text(&line, panel.x + 10.0, y);
            y += if viewport.compact { 22.0 } else { 18.0 };
        }
        return;
    }

    let thumb_size = if viewport.compact { 54.0 } else { 30.0 };
    let item_gap = if viewport.compact { 14.0 } else { 6.0 };
    let label_gap = if viewport.compact { 16.0 } else { 10.0 };
    let usable_width = panel.width - 20.0;
    let cols = ((usable_width + item_gap) / (thumb_size + item_gap)).floor().max(1.0) as usize;
    let start_x = panel.x + 10.0;
    let start_y = panel.y + if viewport.compact { 42.0 } else { 28.0 };

    for (index, mushroom) in app.collected_mushrooms.iter().enumerate() {
        let col = index % cols.max(1);
        let row = index / cols.max(1);
        let x = start_x + col as f64 * (thumb_size + item_gap);
        let y = start_y + row as f64 * (thumb_size + label_gap + item_gap);
        if y + thumb_size > panel.y + panel.height - 18.0 {
            ctx.set_fill_style_str("#8aaa7c");
            ctx.set_font(if viewport.compact {
                "16px Georgia"
            } else {
                "10px Georgia"
            });
            let remaining = app.collected_mushrooms.len() - index;
            let _ = ctx.fill_text(&format!("+{} more", remaining), panel.x + 10.0, panel.y + panel.height - 14.0);
            break;
        }

        if let Some(image) = images.get(&mushroom.image_key) {
            let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(image, x, y, thumb_size, thumb_size);
        } else {
            draw_fallback_sprite(ctx, &mushroom.image_key, x, y, thumb_size);
        }

        ctx.set_fill_style_str("#a8c49a");
        ctx.set_font(if viewport.compact {
            "12px Georgia"
        } else {
            "8px Georgia"
        });
        let short_name = shorten_name(&mushroom.display_name, if viewport.compact { 10 } else { 6 });
        let name_width = short_name.len() as f64 * if viewport.compact { 6.0 } else { 4.2 };
        let _ = ctx.fill_text(
            &short_name,
            x + (thumb_size - name_width) / 2.0,
            y + thumb_size + if viewport.compact { 12.0 } else { 8.0 },
        );
    }
}

fn draw_center_animation(ctx: &CanvasRenderingContext2d, app: &AppState) {
    let Some(ref animation) = app.animation else { return };
    let viewport = app.viewport;
    let layout = ui::play_layout(viewport, app.game.lane_count());
    let center_x = layout.game_center_x;
    let center_y = layout.fall_zone.center_y();

    match animation {
        CenterAnimation::Correct { progress } => {
            let progress = progress.min(1.0);
            let scale = if progress < 0.3 { progress / 0.3 } else { 1.0 };
            let alpha = if progress > 0.5 { 1.0 - (progress - 0.5) * 2.0 } else { 0.9 };
            let size = (if viewport.compact { 108.0 } else { 80.0 }) * scale;

            ctx.save();
            ctx.set_global_alpha(alpha.max(0.0));
            ctx.set_fill_style_str("#4CAF50");
            ctx.set_font(&format!("bold {}px Georgia", size));
            let _ = ctx.fill_text("OK", center_x - size * 0.55, center_y + size * 0.2);
            ctx.restore();
        }
        CenterAnimation::Wrong { progress } => {
            let progress = progress.min(1.0);
            let scale = if progress < 0.4 { progress / 0.4 * 1.5 } else { 1.5 };
            let alpha = if progress > 0.4 { 1.0 - (progress - 0.4) / 0.6 } else { 0.9 };
            let size = (if viewport.compact { 128.0 } else { 100.0 }) * scale;

            ctx.save();
            ctx.set_global_alpha(alpha.max(0.0));
            ctx.set_fill_style_str("#F44336");
            ctx.set_font(&format!("bold {}px Georgia", size));
            let _ = ctx.fill_text("X", center_x - size * 0.3, center_y + size * 0.3);
            ctx.restore();
        }
        CenterAnimation::BasketCollected { progress } => {
            let progress = progress.min(1.0);
            let bounce = if progress < 0.5 {
                (progress * std::f64::consts::PI).sin() * if viewport.compact { 58.0 } else { 40.0 }
            } else {
                ((progress - 0.5) * std::f64::consts::PI * 2.0).sin() * if viewport.compact { 14.0 } else { 10.0 }
            };
            let alpha = if progress > 0.7 { 1.0 - (progress - 0.7) / 0.3 } else { 1.0 };
            let size = if viewport.compact { 92.0 } else { 70.0 };

            ctx.save();
            ctx.set_global_alpha(alpha.max(0.0));
            ctx.set_font(&format!("{}px serif", size));
            let _ = ctx.fill_text("🧺", center_x - size * 0.3, center_y - bounce);
            ctx.restore();
        }
    }
}

fn draw_basket_fact(
    ctx: &CanvasRenderingContext2d,
    app: &AppState,
    mushrooms: &[game_core::FallingMushroom],
    fact: &str,
    images: &ImageCache,
) {
    let viewport = app.viewport;
    ctx.set_fill_style_str("rgba(10, 20, 10, 0.9)");
    ctx.fill_rect(0.0, 0.0, viewport.width, viewport.height);

    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font(if viewport.compact {
        "bold 34px Georgia"
    } else {
        "bold 36px Georgia"
    });
    let _ = ctx.fill_text("Basket Collected!", if viewport.compact { 120.0 } else { 250.0 }, if viewport.compact { 74.0 } else { 82.0 });

    let item_width = if viewport.compact { 120.0 } else { 100.0 };
    let image_size = if viewport.compact { 82.0 } else { 70.0 };
    let total_width = mushrooms.len() as f64 * item_width;
    let start_x = (viewport.width - total_width) / 2.0;
    let image_y = if viewport.compact { 112.0 } else { 112.0 };
    for (index, mushroom) in mushrooms.iter().enumerate() {
        let slot_x = start_x + index as f64 * item_width;
        let image_x = slot_x + (item_width - image_size) / 2.0;
        if let Some(image) = images.get(&mushroom.image_key) {
            let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(image, image_x, image_y, image_size, image_size);
        } else {
            draw_fallback_sprite(ctx, &mushroom.image_key, image_x, image_y, image_size);
        }

        ctx.set_fill_style_str("#f8f3e8");
        ctx.set_font(if viewport.compact {
            "bold 12px Georgia"
        } else {
            "bold 11px Georgia"
        });
        let short_name = shorten_name(&mushroom.display_name, if viewport.compact { 12 } else { 11 });
        let name_width = short_name.len() as f64 * if viewport.compact { 6.2 } else { 5.6 };
        let _ = ctx.fill_text(
            &short_name,
            slot_x + (item_width - name_width) / 2.0,
            image_y + image_size + 16.0,
        );
    }

    let fact_box = if viewport.compact {
        Rect {
            x: 48.0,
            y: 250.0,
            width: viewport.width - 96.0,
            height: 470.0,
        }
    } else {
        Rect {
            x: 80.0,
            y: 220.0,
            width: viewport.width - 160.0,
            height: 220.0,
        }
    };
    ctx.set_fill_style_str("#2a3a2c");
    ctx.fill_rect(fact_box.x, fact_box.y, fact_box.width, fact_box.height);
    ctx.set_stroke_style_str("#4a6a4c");
    ctx.set_line_width(2.0);
    ctx.stroke_rect(fact_box.x, fact_box.y, fact_box.width, fact_box.height);

    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font(if viewport.compact {
        "18px Georgia"
    } else {
        "16px Georgia"
    });
    let line_height = if viewport.compact { 28.0 } else { 24.0 };
    let mut line_y = fact_box.y + if viewport.compact { 42.0 } else { 40.0 };
    for line in wrap_text(fact, if viewport.compact { 42 } else { 70 }) {
        let _ = ctx.fill_text(&line, fact_box.x + 24.0, line_y);
        line_y += line_height;
        if line_y > fact_box.y + fact_box.height - 70.0 {
            break;
        }
    }

    let button = ui::primary_button_rect(viewport);
    draw_button(
        ctx,
        &button,
        "Continue",
        "#f5d67a",
        "#203122",
        viewport.compact,
    );
}

fn draw_level_complete(ctx: &CanvasRenderingContext2d, app: &AppState, images: &ImageCache) {
    let viewport = app.viewport;
    ctx.set_fill_style_str("rgba(10, 20, 10, 0.92)");
    ctx.fill_rect(0.0, 0.0, viewport.width, viewport.height);

    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font(if viewport.compact {
        "bold 40px Georgia"
    } else {
        "bold 36px Georgia"
    });
    let _ = ctx.fill_text(
        &format!("Level {} Complete!", app.current_level + 1),
        if viewport.compact { 120.0 } else { 250.0 },
        if viewport.compact { 76.0 } else { 70.0 },
    );

    ctx.set_fill_style_str("#a8c49a");
    ctx.set_font(if viewport.compact {
        "18px Georgia"
    } else {
        "18px Georgia"
    });
    let _ = ctx.fill_text(
        &format!(
            "All {} mushrooms sorted by {}.",
            app.active_catalog_len(),
            app.category_mode.display_name()
        ),
        if viewport.compact { 104.0 } else { 240.0 },
        if viewport.compact { 108.0 } else { 102.0 },
    );

    let collection_box = if viewport.compact {
        Rect {
            x: 46.0,
            y: 150.0,
            width: viewport.width - 92.0,
            height: 650.0,
        }
    } else {
        Rect {
            x: 58.0,
            y: 132.0,
            width: viewport.width - 116.0,
            height: 310.0,
        }
    };
    ctx.set_fill_style_str("#213224");
    ctx.fill_rect(
        collection_box.x,
        collection_box.y,
        collection_box.width,
        collection_box.height,
    );
    ctx.set_stroke_style_str("#4a6a4c");
    ctx.set_line_width(2.0);
    ctx.stroke_rect(
        collection_box.x,
        collection_box.y,
        collection_box.width,
        collection_box.height,
    );

    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font(if viewport.compact {
        "bold 22px Georgia"
    } else {
        "bold 16px Georgia"
    });
    let _ = ctx.fill_text(
        "Collection Review",
        collection_box.x + 18.0,
        collection_box.y + if viewport.compact { 28.0 } else { 22.0 },
    );

    let thumb_size = if viewport.compact { 60.0 } else { 46.0 };
    let gap = if viewport.compact { 18.0 } else { 12.0 };
    let cols = ((collection_box.width - 30.0 + gap) / (thumb_size + gap)).floor().max(1.0) as usize;
    let start_x = collection_box.x + 18.0;
    let start_y = collection_box.y + if viewport.compact { 50.0 } else { 40.0 };
    for (index, mushroom) in app.collected_mushrooms.iter().enumerate() {
        let col = index % cols.max(1);
        let row = index / cols.max(1);
        let x = start_x + col as f64 * (thumb_size + gap);
        let y = start_y + row as f64 * (thumb_size + gap + if viewport.compact { 16.0 } else { 12.0 });
        if y + thumb_size > collection_box.y + collection_box.height - 54.0 {
            break;
        }

        if let Some(image) = images.get(&mushroom.image_key) {
            let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(image, x, y, thumb_size, thumb_size);
        } else {
            draw_fallback_sprite(ctx, &mushroom.image_key, x, y, thumb_size);
        }

        ctx.set_fill_style_str("#a8c49a");
        ctx.set_font(if viewport.compact {
            "12px Georgia"
        } else {
            "9px Georgia"
        });
        let short_name = shorten_name(&mushroom.display_name, if viewport.compact { 11 } else { 10 });
        let name_width = short_name.len() as f64 * if viewport.compact { 6.0 } else { 5.0 };
        let _ = ctx.fill_text(
            &short_name,
            x + (thumb_size - name_width) / 2.0,
            y + thumb_size + if viewport.compact { 12.0 } else { 10.0 },
        );
    }

    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font(if viewport.compact {
        "bold 20px Georgia"
    } else {
        "bold 18px Georgia"
    });
    let _ = ctx.fill_text(
        &format!("Score {}", app.game.score()),
        collection_box.x + 18.0,
        collection_box.y + collection_box.height - 18.0,
    );

    let button = ui::primary_button_rect(viewport);
    let label = if app.current_level < 3 {
        "Next Level"
    } else {
        "Finish Run"
    };
    draw_button(ctx, &button, label, "#f5d67a", "#203122", viewport.compact);
}

fn draw_pause_overlay(ctx: &CanvasRenderingContext2d, app: &AppState) {
    let viewport = app.viewport;
    ctx.set_fill_style_str("rgba(0, 0, 0, 0.76)");
    ctx.fill_rect(0.0, 0.0, viewport.width, viewport.height);

    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font(if viewport.compact {
        "bold 52px Georgia"
    } else {
        "bold 48px Georgia"
    });
    let _ = ctx.fill_text("Paused", if viewport.compact { 242.0 } else { 370.0 }, if viewport.compact { 240.0 } else { 220.0 });

    ctx.set_fill_style_str("#a8c49a");
    ctx.set_font(if viewport.compact {
        "20px Georgia"
    } else {
        "20px Georgia"
    });
    let _ = ctx.fill_text(
        "Resume with Enter/Esc or click Resume.",
        if viewport.compact { 152.0 } else { 272.0 },
        if viewport.compact { 292.0 } else { 268.0 },
    );

    let secondary = ui::secondary_button_rect(viewport);
    let primary = ui::primary_button_rect(viewport);
    draw_button(ctx, &secondary, "Return to Menu", "#2a3a2c", "#f8f3e8", viewport.compact);
    draw_button(ctx, &primary, "Resume", "#f5d67a", "#203122", viewport.compact);
}

fn draw_game_over(ctx: &CanvasRenderingContext2d, app: &AppState, images: &ImageCache) {
    let viewport = app.viewport;
    ctx.set_fill_style_str("rgba(10, 20, 10, 0.92)");
    ctx.fill_rect(0.0, 0.0, viewport.width, viewport.height);

    ctx.set_fill_style_str("#f8f3e8");
    ctx.set_font(if viewport.compact {
        "bold 42px Georgia"
    } else {
        "bold 44px Georgia"
    });
    let _ = ctx.fill_text(
        "All Levels Complete!",
        if viewport.compact { 84.0 } else { 210.0 },
        if viewport.compact { 80.0 } else { 90.0 },
    );

    ctx.set_fill_style_str("#f5d67a");
    ctx.set_font(if viewport.compact {
        "bold 28px Georgia"
    } else {
        "bold 28px Georgia"
    });
    let _ = ctx.fill_text(
        &format!("Final Score {}", app.game.score()),
        if viewport.compact { 202.0 } else { 330.0 },
        if viewport.compact { 126.0 } else { 142.0 },
    );

    ctx.set_fill_style_str("#8aaa7c");
    ctx.set_font(if viewport.compact {
        "20px Georgia"
    } else {
        "20px Georgia"
    });
    let _ = ctx.fill_text(
        &format!("{} baskets collected", app.total_baskets),
        if viewport.compact { 224.0 } else { 324.0 },
        if viewport.compact { 164.0 } else { 178.0 },
    );
    let _ = ctx.fill_text(
        &format!("{} mushrooms in the basket log", app.collected_mushrooms.len()),
        if viewport.compact { 128.0 } else { 252.0 },
        if viewport.compact { 196.0 } else { 208.0 },
    );

    let grid_box = if viewport.compact {
        Rect {
            x: 48.0,
            y: 236.0,
            width: viewport.width - 96.0,
            height: 520.0,
        }
    } else {
        Rect {
            x: 70.0,
            y: 240.0,
            width: viewport.width - 140.0,
            height: 180.0,
        }
    };
    ctx.set_fill_style_str("#213224");
    ctx.fill_rect(grid_box.x, grid_box.y, grid_box.width, grid_box.height);
    ctx.set_stroke_style_str("#4a6a4c");
    ctx.set_line_width(2.0);
    ctx.stroke_rect(grid_box.x, grid_box.y, grid_box.width, grid_box.height);

    let thumb_size = if viewport.compact { 54.0 } else { 40.0 };
    let gap = if viewport.compact { 14.0 } else { 10.0 };
    let cols = ((grid_box.width - 24.0 + gap) / (thumb_size + gap)).floor().max(1.0) as usize;
    let start_x = grid_box.x + 12.0;
    let start_y = grid_box.y + 16.0;
    for (index, mushroom) in app.collected_mushrooms.iter().enumerate() {
        let col = index % cols.max(1);
        let row = index / cols.max(1);
        let x = start_x + col as f64 * (thumb_size + gap);
        let y = start_y + row as f64 * (thumb_size + gap + if viewport.compact { 16.0 } else { 12.0 });
        if y + thumb_size > grid_box.y + grid_box.height - 24.0 {
            break;
        }
        if let Some(image) = images.get(&mushroom.image_key) {
            let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(image, x, y, thumb_size, thumb_size);
        } else {
            draw_fallback_sprite(ctx, &mushroom.image_key, x, y, thumb_size);
        }
    }

    let button = ui::primary_button_rect(viewport);
    draw_button(
        ctx,
        &button,
        "Return to Menu",
        "#f5d67a",
        "#203122",
        viewport.compact,
    );
}

fn draw_button(
    ctx: &CanvasRenderingContext2d,
    rect: &Rect,
    label: &str,
    fill: &str,
    text: &str,
    compact: bool,
) {
    ctx.set_fill_style_str(fill);
    ctx.fill_rect(rect.x, rect.y, rect.width, rect.height);
    ctx.set_stroke_style_str("#f5d67a");
    ctx.set_line_width(if compact { 3.0 } else { 2.0 });
    ctx.stroke_rect(rect.x, rect.y, rect.width, rect.height);

    ctx.set_fill_style_str(text);
    ctx.set_font(if compact {
        "bold 20px Georgia"
    } else {
        "bold 18px Georgia"
    });
    let label_width = label.len() as f64 * if compact { 9.0 } else { 8.0 };
    let _ = ctx.fill_text(
        label,
        rect.x + (rect.width - label_width) / 2.0,
        rect.y + rect.height / 2.0 + if compact { 7.0 } else { 6.0 },
    );
}

fn shorten_name(name: &str, max_chars: usize) -> String {
    if name.chars().count() <= max_chars {
        name.to_owned()
    } else {
        let mut shortened = name.chars().take(max_chars).collect::<String>();
        shortened.push('.');
        shortened
    }
}

fn draw_mushroom_item(
    ctx: &CanvasRenderingContext2d,
    mushroom: &game_core::FallingMushroom,
    x: f64,
    y: f64,
    size: f64,
    difficulty: Difficulty,
    images: &ImageCache,
    compact: bool,
) {
    match difficulty {
        Difficulty::Normal | Difficulty::Tricky => {
            if let Some(image) = images.get(&mushroom.image_key) {
                let _ = ctx.draw_image_with_html_image_element_and_dw_and_dh(image, x, y, size, size);
            } else {
                draw_fallback_sprite(ctx, &mushroom.image_key, x, y, size);
            }
        }
        Difficulty::Expert => {
            let emoji_size = if compact { size * 0.82 } else { size * 0.72 };
            ctx.set_font(&format!("{}px serif", emoji_size));
            let _ = ctx.fill_text("🍄", x, y + size * 0.8);
        }
    }

    if compact && difficulty == Difficulty::Normal {
        ctx.set_fill_style_str("#f8f3e8");
        ctx.set_font("10px Georgia");
        let short_name = shorten_name(&mushroom.display_name, 7);
        let _ = ctx.fill_text(&short_name, x + size + 4.0, y + size * 0.7);
    }
}

fn draw_fallback_sprite(ctx: &CanvasRenderingContext2d, image_key: &str, x: f64, y: f64, size: f64) {
    let cap_color = match image_key {
        "chanterelle" => "#FFD700",
        "fly-agaric" => "#CC2200",
        "king-bolete" => "#8B4513",
        "oyster" => "#F5F5DC",
        "shiitake" => "#A0522D",
        "turkey-tail" => "#D2691E",
        "honey-fungus" => "#DAA520",
        "chaga" => "#3B2F2F",
        "cordyceps" => "#FF6347",
        "morel" => "#8B7355",
        "death-cap" => "#E8E8E8",
        "reishi" => "#B22222",
        "enoki" => "#FFFACD",
        "lions-mane" => "#FFF8DC",
        "matsutake" => "#D2B48C",
        "maitake" => "#696969",
        "destroying-angel" => "#FFFFFF",
        "porcini" => "#A0522D",
        "chicken-of-woods" => "#FF8C00",
        "shaggy-ink-cap" => "#F0F0F0",
        "penny-bun" => "#8B6914",
        "giant-puffball" => "#FAFAFA",
        "jelly-ear" => "#8B4513",
        "birch-polypore" => "#D3D3D3",
        "false-morel" => "#A0522D",
        "wood-ear" => "#4A3728",
        "agarikon" => "#E8DCC8",
        "jack-o-lantern" => "#FF6600",
        _ => "#808080",
    };

    let radius = size / 2.0;
    let center_x = x + radius;
    let center_y = y + radius * 0.6;

    ctx.set_fill_style_str(cap_color);
    ctx.begin_path();
    let _ = ctx.arc(center_x, center_y, radius * 0.7, std::f64::consts::PI, 0.0);
    ctx.fill();

    ctx.set_fill_style_str("#F5F5DC");
    ctx.fill_rect(center_x - radius * 0.15, center_y, radius * 0.3, radius * 0.7);
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_chars && !current_line.is_empty() {
            lines.push(current_line);
            current_line = word.to_owned();
        } else if current_line.is_empty() {
            current_line = word.to_owned();
        } else {
            current_line.push(' ');
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
