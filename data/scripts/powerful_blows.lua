function on_activate(parent, ability)
  if parent:has_active_mode() then
    return
  end

  effect = parent:create_effect(ability:name())
  effect:deactivate_with(ability)
  effect:add_num_bonus("accuracy", 10)
  effect:add_num_bonus("defense", -10)
  
  if parent:has_ability("devastating_blows") then
    effect:add_damage(3, 8, 5)
	effect:add_num_bonus("crit_multiplier", 0.5)
  else
    effect:add_damage(2, 5, 0)
  end

  gen = parent:create_anim("crossed_swords")
  gen:set_moves_with_parent()
  gen:set_position(gen:param(-0.5), gen:param(-2.5))
  gen:set_particle_size_dist(gen:fixed_dist(1.0), gen:fixed_dist(1.0))
  effect:apply(gen)

  ability:activate(parent)
end
