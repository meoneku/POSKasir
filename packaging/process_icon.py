import os
from PIL import Image, ImageDraw

src_path = r"C:\Users\staff\.gemini\antigravity\brain\8dbab476-242a-406e-8d56-c004f31b807e\pos_kasir_app_icon_v2_1793500927010.jpg"
out_dir = r"d:\Project\PosKasir\packaging"
os.makedirs(out_dir, exist_ok=True)
out_png = os.path.join(out_dir, "app-icon.png")

img = Image.open(src_path).convert("RGBA")
width, height = img.size

# Buat mask sudut membulat (squircle / rounded rectangle) dengan antialiasing 4x
scale = 4
mask_size = (width * scale, height * scale)
mask = Image.new("L", mask_size, 0)
draw = ImageDraw.Draw(mask)

# Radius 210px (standar modern icon squircle)
radius = int(210 * scale)
draw.rounded_rectangle([(0, 0), (mask_size[0] - 1, mask_size[1] - 1)], radius=radius, fill=255)

# Resize mask kembali ke 1024x1024 dengan Resampling LANCZOS untuk sudut super halus
mask = mask.resize((width, height), Image.Resampling.LANCZOS)

# Terapkan mask alpha
img.putalpha(mask)

# Simpan PNG 1024x1024
img.save(out_png, "PNG")
print(f"Berhasil membuat icon master: {out_png}")
