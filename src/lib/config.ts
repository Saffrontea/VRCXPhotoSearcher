export type Language = 'ja' | 'en'

export type Config = {
  feature_flags: {
    update_db_when_startup: boolean
    language: Language
  }
}
