/** 💡️ Bcf inference schema — topic/comment/viewpoint/author counts derived from the topic tree. */

export interface BcfTopicStats {
  topicCount: number;
  commentCount: number;
  viewpointCount: number;
  authorCount: number;
}

export interface BcfInference {
  /** @derived */
  topicStats: BcfTopicStats;
}
