extends CharacterBody3D

# 基本的にプレイヤーと同じだが、複数存在するためIDで管理を行う。
@export var mob_id: int
# グリッドマップ上での現在位置を持っておく
var current_position_2d: Vector2i

@export var speed: float = 2.0
@export var angular_speed: float = 2.0

var target_pos: Vector3
var anim_playing: bool = false
var current_rotation: float = 0.0
var target_rotation: float = 0.0

var current_direction = 0
var rotation_up = Quaternion(0, 0, 0, 1)
var rotation_down = Quaternion(0, 1, 0, 0)
var rotation_left = Quaternion(0, sqrt(2.0), 0, sqrt(2.0))
var rotation_right = Quaternion(0, sqrt(2.0), 0, -sqrt(2.0))

func init_position(pos):
	target_pos = pos
	transform.origin = pos
	velocity = Vector3.ZERO

func set_next_position(pos):
	target_pos = pos

# dir (int): +-1で指定する
func set_next_rotation(dir):
	current_rotation = 0.0
	target_rotation = PI/2 * dir
	current_direction = (current_direction + dir + 4) % 4
	# 回転移動は即時反映とする。
	get_node("Pivot").transform = get_node("Pivot").transform.rotated(Vector3(0.0, 1.0, 0.0), target_rotation)

func set_next_abs_rotation(dir):
	current_direction = dir
	# 回転移動は即時反映とする。
	# current_directionの値に応じた回転行列を作る	
	var identity_transform = Transform3D()
	target_rotation = -PI/4 * current_direction
	get_node("Pivot").transform = identity_transform.rotated(Vector3(0.0, 1.0, 0.0), target_rotation)

func _physics_process(delta):
	# 目標位置に対してスライドするアニメーションを再生するだけ
	# 現在の自分の表示位置が目標とする表示位置に達した場合にanim_playing = falseとする

	# 目標となる姿勢と現在の姿勢の差分をとる
	# 平行移動
	var diff = target_pos - transform.origin

	# 回転移動
	# 現在の開店から移動する方向を考えたいが、
	# 角度は-180 --- +180とした場合、180度付近でブランチをしないとだめになりそう。

	if diff.length() > 0.05:
		# 目標の姿勢に到達していない場合は、velocityを入力。
		#print("current position: ", transform.origin)
		#print("target position: ", target_pos)
		#print("position diff: ", diff)	
	
		var direction = Vector3.ZERO
		#direction.x = next_player_position.x - player_position.x
		#direction.z = next_player_position.y - player_position.y
		direction.x = diff.x
		direction.z = diff.z
		#if Input.is_action_pressed("w"):
			#direction.z -= 1.0
		#if Input.is_action_pressed("a"):
			#direction.x -= 1.0
		#if Input.is_action_pressed("s"):
			#direction.z += 1.0
		#if Input.is_action_pressed("d"):
			#direction.x += 1.0
			
		if direction != Vector3.ZERO:
			direction = direction.normalized()
		
		var target_velocity = Vector3.ZERO
		target_velocity.x = direction.x * speed
		target_velocity.z = direction.z * speed
		
		velocity = target_velocity
		
		anim_playing = true
		
	#elif absf(target_rotation - current_rotation) > 0.01:
		## 回転移動
		#var current_pos = transform.origin
		#var delta_r = target_rotation * angular_speed * delta
		#current_rotation = current_rotation + delta_r
		#get_node("Pivot").transform = get_node("Pivot").transform.rotated(Vector3(0.0, 1.0, 0.0), delta_r)
		#transform.origin = current_pos
		##print("target rotation: ", target_rotation)
		##print("current rotation: ", current_rotation)
		##print(transform.basis)
		##print(transform.origin)
#
		#anim_playing = true
	else:
		# 目標の姿勢に十分到達している場合は、目標姿勢にワープしてvelocityを0にする。
		velocity = Vector3.ZERO
		transform.origin = target_pos
		#if current_direction == 0:
			#transform.basis = Basis(rotation_up)
		#elif current_direction == 1:
			#transform.basis = Basis(rotation_left)
		#elif current_direction == 2:
			#transform.basis = Basis(rotation_down)
		#elif current_direction == 1:
			#transform.basis = Basis(rotation_right)
		#print(transform.origin)  # = 並進ベクトル
		#print(transform.basis)  # = 回転行列
		current_rotation = 0.0
		target_rotation = 0.0
		anim_playing = false
	move_and_slide()

	
