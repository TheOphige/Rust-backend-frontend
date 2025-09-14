import "react-toastify/dist/ReactToastify.css";
import {
  QueryClient,
  QueryClientProvider,
  useQuery,
} from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { useEffect, useState } from "react";
import { toast, ToastContainer } from "react-toastify";
import { twMerge } from "tailwind-merge";
import { getNotesFn } from "./api/noteApi";
import NoteModal from "./components/note.modal";
import CreateNote from "./components/notes/create.note";
import NoteItem from "./components/notes/note.component";
import NProgress from "nprogress";
import type { INotesResponse, IErrorResponse } from "./api/types";
import { AxiosError } from "axios";

function AppContent() {
  const [openNoteModal, setOpenNoteModal] = useState(false);
  const [page, setPage] = useState(1);
  const limit = 10; // Match backend default limit

  const {
    data: notesResponse,
    isLoading,
    isError,
    error,
    isFetching,
  } = useQuery<INotesResponse, AxiosError<IErrorResponse>>({
    queryKey: ["getNotes", page],
    queryFn: () => getNotesFn(page, limit),
    staleTime: 5 * 1000,
    placeholderData: (previousData) => previousData, // Keep previous data during pagination
  });

  // Handle progress bar
  useEffect(() => {
    if (isLoading || isFetching) {
      NProgress.start();
    } else {
      NProgress.done();
    }
  }, [isLoading, isFetching]);

  // Handle errors with toast
  useEffect(() => {
    if (isError && error) {
      const resMessage =
        error.response?.data?.message ||
        error.response?.data?.detail ||
        error.message ||
        "An unexpected error occurred";
      toast(resMessage, {
        type: "error",
        position: "top-right",
      });
      NProgress.done();
    }
  }, [isError, error]);

  const totalPages = notesResponse?.count
    ? Math.ceil(notesResponse.count / limit)
    : 1;

  return (
    <div className="2xl:max-w-[90rem] max-w-[68rem] mx-auto">
      <div className="m-8 grid grid-cols-[repeat(auto-fill,_320px)] gap-7 grid-rows-[1fr]">
        {/* Add new note card */}
        <div className="p-4 min-h-[18rem] bg-white rounded-lg border border-gray-200 shadow-md flex flex-col items-center justify-center">
          <div
            onClick={() => setOpenNoteModal(true)}
            className="flex items-center justify-center h-20 w-20 border-2 border-dashed border-ct-blue-600 rounded-full text-ct-blue-600 text-5xl cursor-pointer"
          >
            <i className="bx bx-plus"></i>
          </div>
          <h4
            onClick={() => setOpenNoteModal(true)}
            className="text-lg font-medium text-ct-blue-600 mt-5 cursor-pointer"
          >
            Add new note
          </h4>
        </div>

        {/* Loading state */}
        {isLoading && (
          <div className="col-span-full text-center text-ct-dark-600">
            Loading notes...
          </div>
        )}

        {/* Error state */}
        {isError && (
          <div className="col-span-full text-center text-red-500">
            Failed to load notes: {error?.message || "Unknown error"}
          </div>
        )}

        {/* Render notes */}
        {!isLoading && !isError && notesResponse?.notes?.length ? (
          notesResponse.notes.map((note) => (
            <NoteItem key={note.id} note={note} />
          ))
        ) : (
          !isLoading &&
          !isError && (
            <div className="col-span-full text-center text-ct-dark-600">
              No notes available
            </div>
          )
        )}

        {/* Pagination controls */}
        {!isLoading && !isError && (
          <div className="col-span-full flex justify-between items-center mt-4">
            <button
              className={twMerge(
                "px-4 py-2 bg-ct-blue-600 text-white rounded disabled:bg-gray-300",
                isFetching && "opacity-50"
              )}
              onClick={() => setPage((old) => Math.max(old - 1, 1))}
              disabled={page === 1 || isFetching}
            >
              Previous
            </button>
            <span className="text-ct-dark-600">
              Page {page} of {totalPages}
            </span>
            <button
              className={twMerge(
                "px-4 py-2 bg-ct-blue-600 text-white rounded disabled:bg-gray-300",
                isFetching && "opacity-50"
              )}
              onClick={() => setPage((old) => old + 1)}
              disabled={page >= totalPages || isFetching}
            >
              Next
            </button>
          </div>
        )}

        {/* Create Note Modal */}
        <NoteModal
          openNoteModal={openNoteModal}
          setOpenNoteModal={setOpenNoteModal}
        >
          <CreateNote setOpenNoteModal={setOpenNoteModal} />
        </NoteModal>
      </div>
    </div>
  );
}

function App() {
  const [queryClient] = useState(() => new QueryClient());
  return (
    <QueryClientProvider client={queryClient}>
      <AppContent />
      <ToastContainer />
      <ReactQueryDevtools initialIsOpen={false} />
    </QueryClientProvider>
  );
}

export default App;