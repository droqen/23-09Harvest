extends CSGBox3D

@export var upper_position : Vector3
@export var lower_position : Vector3
@export var button_area : Area3D

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var target_position = upper_position
	if !button_area.get_overlapping_bodies().is_empty():
		target_position = lower_position
	if Input.is_action_pressed("ui_cancel"):
		target_position = lower_position
	position = lerp(position, target_position, 0.01)
