extends Area3D

func _on_body_entered(_body):
	get_tree().change_scene_to_file("res://10_scene_InsideTheLighthouse/InsideTheLighthouse.tscn")
