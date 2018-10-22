smoke_radius = 5.0

function on_activate(parent, ability)
  targets = parent:targets()
  
  targeter = parent:create_targeter(ability)
  targeter:set_free_select(10.0)
  -- targeter:set_free_select_must_be_passable("1by1")
  targeter:set_shape_circle(smoke_radius)
  targeter:add_all_effectable(targets)
  targeter:activate()
end

function on_target_select(parent, ability, targets)
  ability:activate(parent)

  points = targets:affected_points()
  surface = parent:create_surface(ability:name(), points, ability:duration())
  surface:set_squares_to_fire_on_moved(3)
  surface:add_attribute_bonus("Intellect", -4)
  surface:add_attribute_bonus("Perception", -4)
  
  cb = ability:create_callback(parent)
  cb:set_on_surface_round_elapsed_fn("on_round_elapsed")
  cb:set_on_moved_in_surface_fn("on_moved")
  surface:add_callback(cb)
  
  s_anim = parent:create_particle_generator("particles/circle12")
  s_anim:set_position(s_anim:param(0.0), s_anim:param(0.0))
  s_anim:set_color(s_anim:param(0.0), s_anim:param(1.0), s_anim:param(0.2), s_anim:param(0.3))
  s_anim:set_gen_rate(s_anim:param(20.0))
  s_anim:set_particle_size_dist(s_anim:fixed_dist(1.0), s_anim:fixed_dist(1.0))
  s_anim:set_particle_duration_dist(s_anim:fixed_dist(1.0))
  s_anim:set_particle_position_dist(s_anim:dist_param(s_anim:uniform_dist(-1.0, 1.0), s_anim:uniform_dist(-0.2, 0.2)),
                                    s_anim:dist_param(s_anim:uniform_dist(-1.0, 1.0), s_anim:uniform_dist(-0.2, 0.2)))
  s_anim:set_draw_above_entities()
  surface:add_anim(s_anim)
  surface:apply()
end

function on_moved(parent, ability, targets)
  target = targets:first()
  target:take_damage(parent, 3, 6, "Acid", 8)
end

function on_round_elapsed(parent, ability, targets)
  targets = targets:to_table()
  for i = 1, #targets do
	targets[i]:take_damage(parent, 3, 6, "Acid", 8)
  end
end