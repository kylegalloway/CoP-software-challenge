# win95

**Concept:** desktop + dialog window — a nod to the original challenge mockup (`challenge.png`), rendering the whole page as a classic Windows 95 application window sitting on a desktop.

## Palette

The canonical Win95 grey (`#C0C0C0`/`#ECE9D8`-family) for all window chrome, with the signature 3D bevel effect achieved via two-tone borders (light grey top/left, dark grey bottom/right, or vice versa for "pressed" states) rather than shadows. Title bar in Win95's blue gradient (`#000080`-to-`#1084D0`), white title text. Content area (the actual date/entries) sits on plain white or light grey, high-contrast black text — Win95 UIs were not concerned with subtlety.

## Typography

Tahoma or MS Sans Serif (or the nearest system-font equivalent) at small, dense sizes throughout — no display fonts, no font-weight variation beyond bold for the title bar and maybe section headers. Every visual distinction in this skin comes from borders and background color blocks, not typography.

## Layout

The entire page renders as one centered "window": fake title bar with the site's name and dummy minimize/maximize/close button glyphs (`_ □ ✕`, CSS-drawn, non-functional — no actual window behavior, just the chrome), a thin menu-bar-style strip beneath it (could house the skin picker as a "menu"), then the content area with an inset border (`border-style: inset`) framing the date picker and entries list like a dialog box's content pane. The background *behind* the window (visible page margin) can use the Win95 teal desktop color for full effect.

## Buttons and inputs

Any interactive element (the date input, the skin `<select>`) should get the beveled 3D button treatment consistent with the OS — flat modern inputs would break the illusion immediately. This is the skin where getting the bevel direction right (raised = interactive/idle, sunken = pressed/active) matters most for selling the reference.

## Empty state

Framed as a system dialog: something like "No data found." with a small CSS-drawn icon substitute (a triangle "!" or "i" in a circle, inline SVG or CSS shapes only — no external icon assets) mimicking a Windows message box.

## Why this reinforces the identity

This skin is the most literal callback in the set — it's referencing `challenge.png` directly — so precision on the bevel/chrome details (not just "grey and blocky") is what makes it land as *specifically* Windows 95 rather than generic retro-OS pastiche.
