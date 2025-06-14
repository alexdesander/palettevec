use crate::palette::{hybrid::HybridPalette, Palette};

use super::*;

#[test]
fn palette_insert_new() {
    test_palette_insert_new(HybridPalette::<0, u32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<1, u32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<2, u32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<3, u32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<4, u32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<5, u32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<17, u32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<32, u32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<127, u32>::new(), 2049);
    test_palette_insert_new(HybridPalette::<333, u32>::new(), 3589);
}

#[test]
fn palette_len() {
    test_palette_len(HybridPalette::<0, u32>::new(), 2049);
    test_palette_len(HybridPalette::<1, u32>::new(), 2049);
    test_palette_len(HybridPalette::<2, u32>::new(), 2049);
    test_palette_len(HybridPalette::<3, u32>::new(), 2049);
    test_palette_len(HybridPalette::<4, u32>::new(), 2049);
    test_palette_len(HybridPalette::<5, u32>::new(), 2049);
    test_palette_len(HybridPalette::<17, u32>::new(), 2049);
    test_palette_len(HybridPalette::<32, u32>::new(), 2049);
    test_palette_len(HybridPalette::<127, u32>::new(), 2049);
    test_palette_len(HybridPalette::<333, u32>::new(), 3589);
}

#[test]
fn palette_index_size() {
    test_palette_index_size(HybridPalette::<0, u32>::new(), 2049);
    test_palette_index_size(HybridPalette::<1, u32>::new(), 2049);
    test_palette_index_size(HybridPalette::<2, u32>::new(), 2049);
    test_palette_index_size(HybridPalette::<3, u32>::new(), 2049);
    test_palette_index_size(HybridPalette::<4, u32>::new(), 2049);
    test_palette_index_size(HybridPalette::<5, u32>::new(), 2049);
    test_palette_index_size(HybridPalette::<17, u32>::new(), 2049);
    test_palette_index_size(HybridPalette::<32, u32>::new(), 2049);
    test_palette_index_size(HybridPalette::<127, u32>::new(), 2049);
    test_palette_index_size(HybridPalette::<333, u32>::new(), 3589);
}

#[test]
fn palette_get_by_value() {
    test_pallete_get_by_value(HybridPalette::<0, u32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<1, u32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<2, u32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<3, u32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<4, u32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<5, u32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<17, u32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<32, u32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<127, u32>::new(), 2049);
    test_pallete_get_by_value(HybridPalette::<333, u32>::new(), 3589);
}

#[test]
fn palette_get_by_index() {
    test_palette_get_by_index(HybridPalette::<0, u32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<1, u32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<2, u32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<3, u32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<4, u32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<5, u32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<17, u32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<32, u32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<127, u32>::new(), 2049);
    test_palette_get_by_index(HybridPalette::<333, u32>::new(), 3589);
}

#[test]
fn palette_mark_as_unused() {
    test_palette_mark_as_unused(HybridPalette::<0, u32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<1, u32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<2, u32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<3, u32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<4, u32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<5, u32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<17, u32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<32, u32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<127, u32>::new(), 2049);
    test_palette_mark_as_unused(HybridPalette::<333, u32>::new(), 3589);
}

#[test]
fn palette_mark_as_unused_len() {
    test_palette_mark_as_unused_len(HybridPalette::<0, u32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<1, u32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<2, u32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<3, u32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<4, u32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<5, u32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<17, u32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<32, u32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<127, u32>::new(), 2049);
    test_palette_mark_as_unused_len(HybridPalette::<333, u32>::new(), 3589);
}

#[test]
fn palette_optimize() {
    test_palette_optimize(HybridPalette::<0, u32>::new(), 2049);
    test_palette_optimize(HybridPalette::<1, u32>::new(), 2049);
    test_palette_optimize(HybridPalette::<2, u32>::new(), 2049);
    test_palette_optimize(HybridPalette::<3, u32>::new(), 2049);
    test_palette_optimize(HybridPalette::<4, u32>::new(), 2049);
    test_palette_optimize(HybridPalette::<5, u32>::new(), 2049);
    test_palette_optimize(HybridPalette::<17, u32>::new(), 2049);
    test_palette_optimize(HybridPalette::<32, u32>::new(), 2049);
    test_palette_optimize(HybridPalette::<127, u32>::new(), 2049);
    test_palette_optimize(HybridPalette::<333, u32>::new(), 3589);
}

#[test]
fn palette_index_size_after_optimizing() {
    test_palette_index_size_after_optimizing(HybridPalette::<0, u32>::new(), 16);
    test_palette_index_size_after_optimizing(HybridPalette::<1, u32>::new(), 20);
    test_palette_index_size_after_optimizing(HybridPalette::<2, u32>::new(), 9);
    test_palette_index_size_after_optimizing(HybridPalette::<3, u32>::new(), 7);
    test_palette_index_size_after_optimizing(HybridPalette::<4, u32>::new(), 27);
    test_palette_index_size_after_optimizing(HybridPalette::<5, u32>::new(), 29);
    test_palette_index_size_after_optimizing(HybridPalette::<17, u32>::new(), 18);
    test_palette_index_size_after_optimizing(HybridPalette::<32, u32>::new(), 2);
    test_palette_index_size_after_optimizing(HybridPalette::<127, u32>::new(), 1);
    test_palette_index_size_after_optimizing(HybridPalette::<333, u32>::new(), 249);
}

#[test]
fn palette_iter() {
    test_palette_iter(HybridPalette::<0, u32>::new(), 16);
    test_palette_iter(HybridPalette::<1, u32>::new(), 20);
    test_palette_iter(HybridPalette::<2, u32>::new(), 9);
    test_palette_iter(HybridPalette::<3, u32>::new(), 7);
    test_palette_iter(HybridPalette::<4, u32>::new(), 27);
    test_palette_iter(HybridPalette::<5, u32>::new(), 29);
    test_palette_iter(HybridPalette::<17, u32>::new(), 43);
    test_palette_iter(HybridPalette::<32, u32>::new(), 498);
    test_palette_iter(HybridPalette::<127, u32>::new(), 10);
    test_palette_iter(HybridPalette::<333, u32>::new(), 20);
    test_palette_iter(HybridPalette::<200, u32>::new(), 43);
}

#[test]
fn palette_iter_mut() {
    test_palette_iter_mut(HybridPalette::<0, u32>::new(), 16);
    test_palette_iter_mut(HybridPalette::<1, u32>::new(), 20);
    test_palette_iter_mut(HybridPalette::<2, u32>::new(), 9);
    test_palette_iter_mut(HybridPalette::<3, u32>::new(), 7);
    test_palette_iter_mut(HybridPalette::<4, u32>::new(), 27);
    test_palette_iter_mut(HybridPalette::<5, u32>::new(), 29);
    test_palette_iter_mut(HybridPalette::<17, u32>::new(), 43);
    test_palette_iter_mut(HybridPalette::<32, u32>::new(), 498);
    test_palette_iter_mut(HybridPalette::<127, u32>::new(), 10);
    test_palette_iter_mut(HybridPalette::<333, u32>::new(), 20);
    test_palette_iter_mut(HybridPalette::<200, u32>::new(), 43);
}
