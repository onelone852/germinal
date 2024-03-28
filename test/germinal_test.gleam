import germinal
import germinal/types
import gleeunit
import gleam/io  
import gleam/result

pub fn main() {
  gleeunit.main()
}

pub fn raw_mode_test() {
    use _ <- result.try(germinal.enable_raw_mode())
    use res <- result.try(germinal.read())
    case res {
      types.Key(key) -> {
        let modifiers = types.get_key_modifiers(key)
        io.debug(modifiers)
        io.debug(types.contains_modifiers(modifiers, types.alt_modifier()))
        Nil
      }
      _ -> Nil
    }
    use _ <- result.try(germinal.disable_raw_mode())
    Ok(Nil)
}