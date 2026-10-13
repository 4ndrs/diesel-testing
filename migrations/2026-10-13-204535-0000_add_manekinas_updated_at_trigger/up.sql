-- Your SQL goes here
CREATE TRIGGER manekinas_updated_at
AFTER UPDATE OF name, hair_color ON manekinas
FOR EACH ROW
BEGIN
  UPDATE manekinas
  SET updated_at = CURRENT_TIMESTAMP
  WHERE id = NEW.id;
END;
