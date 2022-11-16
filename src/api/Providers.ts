export enum Providers {
  OneDrive = "OneDrive",
  GoogleDrive = "Google",
  S3 = "S3",
  SQL = "SQL",
  Native = "Native"
}

export interface ProviderId {
  id: string
  type: Providers
  data: any
}