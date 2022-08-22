import { faDocker } from "@fortawesome/free-brands-svg-icons";
import {
  faBox,
  faArrowDown,
  faFileZipper,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useQuery } from "@tanstack/react-query";
import { getArtifactsOfProject } from "../../api/projects";
import { Artifact } from "../../api/artifacts";
import { Link, useMatch } from "@tanstack/react-location";
import { ArtifactEntry } from "../../components/ArtifactEntry";

const Artifacts = () => {
  const {
    params: { projectID },
  } = useMatch();
  const query = useQuery(["artifacts", projectID], ({ queryKey }) =>
    getArtifactsOfProject(queryKey[1])
  );
  if (!query.data) return <></>;
  const artifacts = query.data as Artifact[];
  console.log;
  return (
    <>
      <p className="text-3xl font-bold mb-3 text-gray-200">Artifacts</p>

      <div className="flex divide-y-[1px] divide-neutral-700 flex-col">
       {/*  <div className="flex gap-5 items-center py-2">
          <FontAwesomeIcon icon={faBox} fixedWidth className="text-lg" />
          <div className="flex flex-col">
            <p>neko.rpm</p>
            <p className="text-xs font-light">
              12MB • <code>v1.2.0</code> • 6h
            </p>
          </div>
          <FontAwesomeIcon icon={faArrowDown} className="ml-auto text-lg" />
        </div> */}
        {ArtifactEntry(artifacts)}
        {/* {query.data.map((artifact: Artifact) => (
          <div className="flex gap-5 items-center py-2">
            <FontAwesomeIcon icon={faBox} fixedWidth className="text-lg" />
            <div className="flex flex-col">
              <p>{artifact.name}</p>
              <p className="text-xs font-light">
                12MB • <code>v1.2.0</code> • 6h
              </p>
            </div>
            <a href={artifact.url} className="ml-auto text-lg">
            <FontAwesomeIcon icon={faArrowDown} className="ml-auto text-lg" />
            </a>
          </div>
        ))} */}
{/*         <div className="flex gap-5 items-center py-2">
          <FontAwesomeIcon icon={faBox} fixedWidth className="text-lg" />
          <div className="flex flex-col">
            <p>neko-devel.rpm</p>
            <p className="text-xs font-light">
              88MB • <code>v1.2.0</code> • 2h
            </p>
          </div>
          <FontAwesomeIcon icon={faArrowDown} className="ml-auto text-lg" />
        </div>
        <div className="flex gap-5 items-center py-2">
          <FontAwesomeIcon icon={faDocker} fixedWidth className="text-lg" />
          <div className="flex flex-col">
            <p>neko</p>
            <p className="text-xs font-light">
              102MB • <code>latest</code> • 12h
            </p>
          </div>
          <FontAwesomeIcon icon={faArrowDown} className="ml-auto text-lg" />
        </div>
        <div className="flex gap-5 items-center py-2">
          <FontAwesomeIcon icon={faFileZipper} fixedWidth className="text-lg" />
          <div className="flex flex-col">
            <p>neko.tar</p>
            <p className="text-xs font-light">
              55MB • <code>02ff55</code> • 3h
            </p>
          </div>
          <FontAwesomeIcon icon={faArrowDown} className="ml-auto text-lg" />
        </div>
 */}      </div>
    </>
  );
};

export default Artifacts;
