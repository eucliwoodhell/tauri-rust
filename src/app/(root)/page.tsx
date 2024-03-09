import { currentUser } from "@clerk/nextjs";
import React from "react";

export default async function Page() {
  const user = await currentUser();
  return (
    <React.Fragment>hola</React.Fragment>
  )
}
