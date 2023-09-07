pub const CONTENTS: &str = r#"

[gd_scene load_steps=3 format=3 uid="uid://ccsamb6jupayd"]

[ext_resource type="PackedScene" uid="uid://bcy2vppkogsi2" path="res://four-sided-build.glb" id="1_i5qtq"]

[sub_resource type="BoxMesh" id="BoxMesh_ggqbh"]

[node name="MyCityTest" type="Node3D"]

[node name="cube1" type="MeshInstance3D" parent="."]
transform = Transform3D(10, 0, 0, 0, 10, 0, 0, 0, 10, 0, -5, 0)
mesh = SubResource("BoxMesh_ggqbh")

[node name="four-sided-build" parent="." instance=ExtResource("1_i5qtq")]
transform = Transform3D(9.70113, 0, 0, 0, 25.2469, 0, 0, 0, 9.57016, -18.9836, 9.53674e-07, 40.9655)

[node name="four-sided-build2" parent="." instance=ExtResource("1_i5qtq")]
transform = Transform3D(9.70113, 0, 0, 0, 25.2469, 0, 0, 0, 9.57016, -3.73036, 0, 2.52592)

[node name="four-sided-build3" parent="." instance=ExtResource("1_i5qtq")]
transform = Transform3D(9.70113, 0, 0, 0, 25.2469, 0, 0, 0, 9.57016, 5.96343, 0, 42.1991)

"#;
