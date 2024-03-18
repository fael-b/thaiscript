import { Link, createBrowserRouter } from "react-router-dom";
import { HomeView } from "../views/Home";
import { ReviewingView } from "../views/Reviewing";

export const appRouter = createBrowserRouter([
  {
    path: "/",
    element: <HomeView />,
    errorElement: <div>404 Not Found</div>,
  },
  {
    path: "/review",
    element: <ReviewingView />,
    errorElement: (
      <div>
        404 Not Found<Link to="/">HOME</Link>
      </div>
    ),
  },
]);
