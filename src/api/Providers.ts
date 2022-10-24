export enum Providers {
  GoogleDrive = "Google",
  S3 = "S3",
  SQL = "SQL"
}

export interface ProviderId {
  id: string
  type: Providers
  data: any
}