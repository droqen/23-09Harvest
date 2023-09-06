pub const CONTENTS: &str = r#"

[gd_scene load_steps=14 format=3 uid="uid://cwcdxl4xkjgwh"]

[ext_resource type="PackedScene" uid="uid://drliciusro4t" path="res://MSH_Building_002.glb" id="1_3kf6n"]
[ext_resource type="PackedScene" uid="uid://bf4iy4xf866fb" path="res://MSH_Building_001.glb" id="2_mr7y2"]
[ext_resource type="PackedScene" uid="uid://ccp2hwmnal508" path="res://MSH_Building_003.glb" id="3_2pifs"]
[ext_resource type="PackedScene" uid="uid://di388pshat38s" path="res://MSH_Building_004.glb" id="4_85ima"]
[ext_resource type="PackedScene" uid="uid://dregsku77s1c5" path="res://MSH_Building_005.glb" id="5_8n6s1"]
[ext_resource type="PackedScene" uid="uid://b6he0ah83p4c5" path="res://MSH_Building_006.glb" id="6_pht7i"]
[ext_resource type="PackedScene" uid="uid://4he81ndu7i5r" path="res://MSH_Building_009.glb" id="7_5jw0x"]
[ext_resource type="PackedScene" uid="uid://cpt6qgl6kpp0h" path="res://MSH_Building_Hole_001.glb" id="8_47pxc"]
[ext_resource type="PackedScene" uid="uid://endburaeyfbj" path="res://MSH_Building_Hole_002.glb" id="9_fopxv"]
[ext_resource type="PackedScene" uid="uid://dtwcjaupcm6ms" path="res://MSH_Building_Hole_003.glb" id="10_xdob0"]
[ext_resource type="PackedScene" uid="uid://7bh2hh72uacb" path="res://MSH_Building_Hole_004.glb" id="11_luqa8"]
[ext_resource type="PackedScene" uid="uid://bv8n3mld1iheb" path="res://MSH_Building_Hole_005.glb" id="12_krffe"]

[sub_resource type="BoxMesh" id="BoxMesh_nts8p"]

[node name="edit_scene_test" type="Node3D"]

[node name="MSH_Building_002" parent="." instance=ExtResource("1_3kf6n")]
transform = Transform3D(2.12806, 0, 0, 0, 2.12806, 0, 0, 0, 2.12806, -1.16, 0, -20.082)

[node name="MSH_Building_001" parent="." instance=ExtResource("2_mr7y2")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 4.66961, 0, 0.831661)

[node name="MSH_Building_003" parent="." instance=ExtResource("3_2pifs")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -11.9015, 0, -8.84082)

[node name="MSH_Building_004" parent="." instance=ExtResource("4_85ima")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -16.4668, 0, 1.52997)

[node name="MSH_Building_005" parent="." instance=ExtResource("5_8n6s1")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 28.5169, 0, 1.91267)

[node name="MSH_Building_006" parent="." instance=ExtResource("6_pht7i")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 18.904, 0, -9.04881)

[node name="MSH_Building_009" parent="." instance=ExtResource("7_5jw0x")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 17.1189, -1.90735e-06, -23.1044)

[node name="MSH_Building_Hole_001" parent="." instance=ExtResource("8_47pxc")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -46.022, 0, 0)

[node name="MSH_Building_Hole_002" parent="." instance=ExtResource("9_fopxv")]
transform = Transform3D(0.541635, 0, -0.840614, 0, 1, 0, 0.840614, 0, 0.541635, -30.4228, 0, -5.94527)

[node name="MSH_Building_Hole_003" parent="." instance=ExtResource("10_xdob0")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -44.1806, 0, 14.2822)

[node name="MSH_Building_Hole_004" parent="." instance=ExtResource("11_luqa8")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -41.5044, 3.8147e-06, 21.9606)

[node name="MSH_Building_Hole_005" parent="." instance=ExtResource("12_krffe")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -32.8307, -3.8147e-06, 24.0764)

[node name="cube1" type="MeshInstance3D" parent="."]
transform = Transform3D(8, 0, 0, 0, 8, 0, 0, 0, 8, -4.374, 4, 47.972)
mesh = SubResource("BoxMesh_nts8p")
[gd_scene load_steps=14 format=3 uid="uid://cwcdxl4xkjgwh"]

[ext_resource type="PackedScene" uid="uid://drliciusro4t" path="res://MSH_Building_002.glb" id="1_3kf6n"]
[ext_resource type="PackedScene" uid="uid://bf4iy4xf866fb" path="res://MSH_Building_001.glb" id="2_mr7y2"]
[ext_resource type="PackedScene" uid="uid://ccp2hwmnal508" path="res://MSH_Building_003.glb" id="3_2pifs"]
[ext_resource type="PackedScene" uid="uid://di388pshat38s" path="res://MSH_Building_004.glb" id="4_85ima"]
[ext_resource type="PackedScene" uid="uid://dregsku77s1c5" path="res://MSH_Building_005.glb" id="5_8n6s1"]
[ext_resource type="PackedScene" uid="uid://b6he0ah83p4c5" path="res://MSH_Building_006.glb" id="6_pht7i"]
[ext_resource type="PackedScene" uid="uid://4he81ndu7i5r" path="res://MSH_Building_009.glb" id="7_5jw0x"]
[ext_resource type="PackedScene" uid="uid://cpt6qgl6kpp0h" path="res://MSH_Building_Hole_001.glb" id="8_47pxc"]
[ext_resource type="PackedScene" uid="uid://endburaeyfbj" path="res://MSH_Building_Hole_002.glb" id="9_fopxv"]
[ext_resource type="PackedScene" uid="uid://dtwcjaupcm6ms" path="res://MSH_Building_Hole_003.glb" id="10_xdob0"]
[ext_resource type="PackedScene" uid="uid://7bh2hh72uacb" path="res://MSH_Building_Hole_004.glb" id="11_luqa8"]
[ext_resource type="PackedScene" uid="uid://bv8n3mld1iheb" path="res://MSH_Building_Hole_005.glb" id="12_krffe"]

[sub_resource type="BoxMesh" id="BoxMesh_nts8p"]

[node name="edit_scene_test" type="Node3D"]

[node name="MSH_Building_002" parent="." instance=ExtResource("1_3kf6n")]
transform = Transform3D(2.12806, 0, 0, 0, 2.12806, 0, 0, 0, 2.12806, -1.16, 0, -20.082)

[node name="MSH_Building_001" parent="." instance=ExtResource("2_mr7y2")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 4.66961, 0, 0.831661)

[node name="MSH_Building_003" parent="." instance=ExtResource("3_2pifs")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -11.9015, 0, -8.84082)

[node name="MSH_Building_004" parent="." instance=ExtResource("4_85ima")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -16.4668, 0, 1.52997)

[node name="MSH_Building_005" parent="." instance=ExtResource("5_8n6s1")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 28.5169, 0, 1.91267)

[node name="MSH_Building_006" parent="." instance=ExtResource("6_pht7i")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 18.904, 0, -9.04881)

[node name="MSH_Building_009" parent="." instance=ExtResource("7_5jw0x")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 17.1189, -1.90735e-06, -23.1044)

[node name="MSH_Building_Hole_001" parent="." instance=ExtResource("8_47pxc")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -46.022, 0, 0)

[node name="MSH_Building_Hole_002" parent="." instance=ExtResource("9_fopxv")]
transform = Transform3D(0.541635, 0, -0.840614, 0, 1, 0, 0.840614, 0, 0.541635, -30.4228, 0, -5.94527)

[node name="MSH_Building_Hole_003" parent="." instance=ExtResource("10_xdob0")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -44.1806, 0, 14.2822)

[node name="MSH_Building_Hole_004" parent="." instance=ExtResource("11_luqa8")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -41.5044, 3.8147e-06, 21.9606)

[node name="MSH_Building_Hole_005" parent="." instance=ExtResource("12_krffe")]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -32.8307, -3.8147e-06, 24.0764)

[node name="cube1" type="MeshInstance3D" parent="."]
transform = Transform3D(8, 0, 0, 0, 8, 0, 0, 0, 8, -4.374, 4, 47.972)
mesh = SubResource("BoxMesh_nts8p")

"#;
