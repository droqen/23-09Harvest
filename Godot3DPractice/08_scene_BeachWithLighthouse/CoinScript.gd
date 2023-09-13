extends Node3D

var collected = false

func _process(delta):
	if collected and not $AudioStreamPlayer.playing:
		queue_free()
	rotate_y(10 * delta)

func _on_area_3d_body_entered(body):
	if not collected:
		$AudioStreamPlayer.pitch_scale = randf_range(0.9,1.1)
		$AudioStreamPlayer.play()
		collected = true
		$coin.hide()
