pub mod loop_clause;
pub mod if_clause;
pub mod while_clause;
pub mod for_clause;
pub mod match_clause;

use loop_clause::*;
use if_clause::*;
use while_clause::*;
use for_clause::*;
use match_clause::*;

pub fn loop_clause() {
    loop_return();
    break_continue();
    labels();
}

pub fn if_clause() {
    if_return();
    if_let();
}

pub fn while_clause() {
    common_use_case();
}

pub fn for_clause() {
    for_in_range();
    for_match();
}

pub fn match_clause() {
    switch();
    pattern_matching();
    match_reference();
    guard();
    bind();
}