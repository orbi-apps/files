import { invoke } from "@tauri-apps/api/tauri"
import type { GenericObject, ObjectId } from "../object"
import type { ProviderId } from "../Providers"

export const listProviders = async (): Promise<any[]> => {
    return await invoke("list_providers")
}

export const addProvider = async (providerId: ProviderId, data: any) => {
    const args = {providerId: provider(providerId), credentials: JSON.stringify(data)}
    return await invoke("add_provider", args)
}

export const listFolderContent = async (objectId: ObjectId, providerId: ProviderId): Promise<any[]> => {
    const x = await invoke("list_folder_content", {providerId: provider(providerId), path: path(objectId)})
    return x as any[]
}

export const readFile = async (objectId: ObjectId, providerId: ProviderId): Promise<any[]> => {
    console.log(path(objectId))
    return await invoke("read_file", {providerId: provider(providerId), path: path(objectId)})
}

export const create = async (parentId: ObjectId, providerId: ProviderId, file: GenericObject) => {
    return await invoke("create", {providerId: provider(providerId), path: path(parentId), file: {
        id: file.id.toString(),
        name: file.name,
        mime_type: file.id.mimeType.toString()
    }})
}

export const write_file = async (objectId: ObjectId, providerId: ProviderId): Promise<any[]> => {
    return await invoke("write_file", {providerId: provider(providerId), path: path(objectId)})
}

export const open = async (objectId: ObjectId, providerId: ProviderId) => {
    console.log(path(objectId))
    return await invoke("open", {providerId: provider(providerId), path: path(objectId)})
}

export const rename = async (objectId: ObjectId, providerId: ProviderId, newName: string) => {
    return await invoke("rename", {providerId: provider(providerId), path: path(objectId), newName})
}

export const move_to = async (objectId: ObjectId, providerId: ProviderId, newParentId: ObjectId, newProviderId: ProviderId) => {
    console.log("------------")
    console.log(objectId)
    console.log(providerId)
    console.log(newParentId)
    console.log(newProviderId)
    return await invoke("move_to", {providerId: provider(providerId), path: path(objectId), newPath: path(newParentId), newProviderId: provider(newProviderId)})
}

export const del = async (objectId: ObjectId, providerId: ProviderId) => {
    return await invoke("delete", {providerId: provider(providerId), path: path(objectId)})
}

const provider = (id: ProviderId) => {
    return {id: id?.id, provider_type: id?.type, data: id.data}
}

const path = (path: ObjectId) => {
    return { path: path.id, mime_type: path.mimeType.toString()}
}