import axios from "axios";
import type { CreateNoteInput } from "../components/notes/create.note";
import type { UpdateNoteInput } from "../components/notes/update.note";
import type { IGenericResponse, INoteResponse, INotesResponse } from "./types";

const BASE_URL = "http://localhost:8080/api/v1/";

export const noteApi = axios.create({
  baseURL: BASE_URL,
  withCredentials: true,
});

noteApi.defaults.headers.common["Content-Type"] = "application/json";

// Create a new note
export const createNoteFn = async (note: CreateNoteInput) => {
  const response = await noteApi.post<INoteResponse>("notes", note);
  return response.data;
};

// Update a note by ID
export const updateNoteFn = async (noteId: string, note: UpdateNoteInput) => {
  const response = await noteApi.patch<INoteResponse>(`notes/${noteId}`, note);
  return response.data;
};

// Delete a note by ID
export const deleteNoteFn = async (noteId: string) => {
  const response = await noteApi.delete<IGenericResponse>(`notes/${noteId}`);
  return response.data;
};

// Get a single note by ID
export const getSingleNoteFn = async (noteId: string) => {
  const response = await noteApi.get<INoteResponse>(`notes/${noteId}`);
  return response.data;
};

// Get paginated list of notes
export const getNotesFn = async (page = 1, limit = 10) => {
  const response = await noteApi.get<INotesResponse>(
    `notes?page=${page}&limit=${limit}`
  );
  return response.data;
};