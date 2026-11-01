import os
from PIL import Image, ImageDraw

def create_pos_icon(output_path, size=1024):
    scale = 4
    canvas_size = size * scale  # 4096 x 4096
    img = Image.new("RGBA", (canvas_size, canvas_size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # 1. Background: Hijau Emerald (#059669) Squircle Bersih
    margin = 36 * scale
    radius = 210 * scale
    bg_box = [margin, margin, canvas_size - margin, canvas_size - margin]
    emerald_color = (5, 150, 105, 255) # Emerald Green #059669
    draw.rounded_rectangle(bg_box, radius=radius, fill=emerald_color)

    # 2. Komputer Sederhana Warna Putih (#FFFFFF)
    # Dimensi Monitor
    mon_w = 620 * scale
    mon_h = 430 * scale
    mon_x1 = (canvas_size - mon_w) // 2
    mon_y1 = int(210 * scale)
    mon_x2 = mon_x1 + mon_w
    mon_y2 = mon_y1 + mon_h
    mon_radius = 42 * scale

    white_color = (255, 255, 255, 255)
    # Gambar Frame Komputer Putih
    draw.rounded_rectangle([mon_x1, mon_y1, mon_x2, mon_y2], radius=mon_radius, fill=white_color)

    # Kaki Penyangga Monitor (Stand Putih)
    stand_w = 76 * scale
    stand_h = 105 * scale
    stand_x1 = (canvas_size - stand_w) // 2
    stand_y1 = mon_y2
    stand_x2 = stand_x1 + stand_w
    stand_y2 = stand_y1 + stand_h
    draw.rectangle([stand_x1, stand_y1, stand_x2, stand_y2], fill=white_color)

    # Tatakan Bawah Stand (Base Putih Melengkung Halus)
    base_w = 280 * scale
    base_h = 32 * scale
    base_x1 = (canvas_size - base_w) // 2
    base_y1 = stand_y2
    base_x2 = base_x1 + base_w
    base_y2 = base_y1 + base_h
    base_radius = 16 * scale
    draw.rounded_rectangle([base_x1, base_y1, base_x2, base_y2], radius=base_radius, fill=white_color)

    # Area Kaca Layar Monitor (Screen Display)
    # Putih bersih dengan garis border presisi (slate-200) agar bezel monitor tampak nyata
    bezel_top = 28 * scale
    bezel_side = 28 * scale
    bezel_bottom = 44 * scale # Bezel bawah sedikit lebih tebal khas monitor modern
    screen_x1 = mon_x1 + bezel_side
    screen_y1 = mon_y1 + bezel_top
    screen_x2 = mon_x2 - bezel_side
    screen_y2 = mon_y2 - bezel_bottom
    screen_radius = 24 * scale

    # Garis pembatas bezel layar
    screen_border_color = (226, 232, 240, 255) # Slate-200
    draw.rounded_rectangle(
        [screen_x1, screen_y1, screen_x2, screen_y2],
        radius=screen_radius,
        fill=white_color,
        outline=screen_border_color,
        width=int(4 * scale)
    )

    # Titik kecil power LED di bezel bawah
    chin_center_y = (screen_y2 + mon_y2) // 2
    led_r = int(3.5 * scale)
    led_color = (203, 213, 225, 255) # Slate-300
    draw.ellipse(
        [(canvas_size // 2) - led_r, chin_center_y - led_r, (canvas_size // 2) + led_r, chin_center_y + led_r],
        fill=led_color
    )

    # 3. Line Barcode Warna Hitam di Tengah Layar
    screen_center_x = (screen_x1 + screen_x2) // 2
    screen_center_y = (screen_y1 + screen_y2) // 2
    bc_h = 180 * scale
    bc_y1 = screen_center_y - (bc_h // 2)
    bc_y2 = bc_y1 + bc_h

    black_color = (15, 23, 42, 255) # Solid jet black / slate-900

    # Pola garis barcode khas ritel POS (tebal & tipis bervariasi teratur)
    pattern = [
        (2, 2), (1, 1), (3, 2), (1, 1), (2, 2), (3, 1), (1, 2),
        (2, 1), (1, 2), (3, 2), (2, 1), (1, 2), (3, 1), (2, 2),
        (1, 1), (3, 2), (1, 2), (2, 1), (3, 2), (1, 1), (2, 2)
    ]
    
    unit_w = 6 * scale
    total_bc_w = sum((bar + sp) * unit_w for bar, sp in pattern) - (pattern[-1][1] * unit_w)
    start_bc_x = screen_center_x - (total_bc_w // 2)

    cur_x = start_bc_x
    for bar_units, space_units in pattern:
        bar_w = bar_units * unit_w
        draw.rounded_rectangle([cur_x, bc_y1, cur_x + bar_w, bc_y2], radius=2 * scale, fill=black_color)
        cur_x += bar_w + (space_units * unit_w)

    # Resize gambar dengan kualitas tinggi (LANCZOS) ke target 1024x1024
    result = img.resize((size, size), Image.Resampling.LANCZOS)
    result.save(output_path, "PNG")
    print(f"Icon master berhasil dibuat: {output_path}")

if __name__ == "__main__":
    out_file = r"d:\Project\PosKasir\packaging\app-icon.png"
    create_pos_icon(out_file)
